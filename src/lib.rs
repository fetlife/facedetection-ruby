use magnus::{error::Result, Error, RArray, Ruby};

#[cfg(feature = "opencv")]
fn detect_opencv(ruby: &Ruby, content: Vec<u8>) -> Result<RArray> {
    use opencv::{core::Size, imgcodecs, imgproc, objdetect, prelude::*, types};

    let cascade_file_path =
        opencv::core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;
    let mut classifier = objdetect::CascadeClassifier::new(&cascade_file_path).map_err(|e| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("failed creating classifier: {}", e),
        )
    })?;
    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgproc::COLOR_BGR2GRAY)
        .map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("failed decoding image: {}", e),
            )
        })?;
    let mut faces = types::VectorOfRect::new();
    classifier
        .detect_multi_scale(
            &img,
            &mut faces,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            Size::new(30, 30),
            Size::new(500, 500),
        )
        .map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("Failed to run detect_multi_scale: {}", e),
            )
        })?;

    let result = ruby.ary_new();
    for face in faces {
        let hash = ruby.hash_new();
        hash.aset(ruby.to_symbol("x"), ruby.integer_from_i64(face.x as i64))?;
        hash.aset(ruby.to_symbol("y"), ruby.integer_from_i64(face.y as i64))?;
        hash.aset(
            ruby.to_symbol("width"),
            ruby.integer_from_i64(face.width as i64),
        )?;
        hash.aset(
            ruby.to_symbol("height"),
            ruby.integer_from_i64(face.height as i64),
        )?;
        result.push(hash)?;
    }
    Ok(result)
}

#[cfg(all(feature = "libfacedetection", feature = "opencv"))]
fn detect_libfacedetection(ruby: &Ruby, content: Vec<u8>) -> Result<RArray> {
    use opencv::{imgcodecs, prelude::*, types};

    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgcodecs::IMREAD_COLOR)
        .map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("unable to decode image: {}", e),
            )
        })?;
    detect_libfacedetection_data(
        ruby,
        img.ptr(0).map_err(|e| {
            Error::new(
                ruby.exception_runtime_error(),
                format!("unable to read image data: {}", e),
            )
        })?,
        img.cols(),
        img.rows(),
        Some(img.mat_step().get(0) as u32),
    )
}

#[cfg(not(feature = "opencv"))]
fn detect_opencv(ruby: &Ruby, _content: Vec<u8>) -> Result<RArray> {
    Err(Error::new(
        ruby.exception_runtime_error(),
        "OpenCV is not enabled",
    ))
}

#[cfg(not(all(feature = "libfacedetection", feature = "opencv")))]
fn detect_libfacedetection(ruby: &Ruby, _content: Vec<u8>) -> Result<RArray> {
    Err(Error::new(
        ruby.exception_runtime_error(),
        "need to have both libfacedetection and OpenCV enabled",
    ))
}

#[cfg(feature = "libfacedetection")]
fn detect_libfacedetection_data(
    ruby: &Ruby,
    bgrdata: *const u8,
    width: i32,
    height: i32,
    step: Option<u32>,
) -> Result<RArray> {
    let facedetect_result = libfacedetection_rs::facedetect_cnn(
        bgrdata,
        width,
        height,
        step.unwrap_or((width * 3) as u32), // calculate step without padding
    )
    .map_err(|e| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("libfacedetection error: {}", e),
        )
    })?;
    let result = ruby.ary_new();
    for face in facedetect_result.faces {
        let hash = ruby.hash_new();
        let landmarks = ruby.ary_new();
        hash.aset(ruby.to_symbol("x"), ruby.integer_from_i64(face.x as i64))?;
        hash.aset(ruby.to_symbol("y"), ruby.integer_from_i64(face.y as i64))?;
        hash.aset(
            ruby.to_symbol("width"),
            ruby.integer_from_i64(face.width as i64),
        )?;
        hash.aset(
            ruby.to_symbol("height"),
            ruby.integer_from_i64(face.height as i64),
        )?;
        hash.aset(
            ruby.to_symbol("confidence"),
            ruby.integer_from_i64(face.confidence as i64),
        )?;
        for landmark in face.landmarks {
            let a = ruby.ary_new();
            a.push(ruby.integer_from_i64(landmark.0 as i64))?;
            a.push(ruby.integer_from_i64(landmark.1 as i64))?;
            landmarks.push(a)?;
        }
        hash.aset(ruby.to_symbol("landmarks"), landmarks)?;
        result.push(hash)?;
    }
    Ok(result)
}

fn pub_detect_opencv(ruby: &Ruby, content: String) -> Result<RArray> {
    detect_opencv(ruby, content.into_bytes())
}

fn pub_detect_libfacedetection(ruby: &Ruby, content: String) -> Result<RArray> {
    detect_libfacedetection(ruby, content.into_bytes())
}

fn pub_detect_libfacedetection_image_data(
    ruby: &Ruby,
    data_ptr: usize,
    width: usize,
    height: usize,
) -> Result<RArray> {
    detect_libfacedetection_data(
        ruby,
        data_ptr as *const u8,
        width as i32,
        height as i32,
        None,
    )
}

fn features(ruby: &Ruby) -> Result<RArray> {
    let result = ruby.ary_new();
    #[cfg(feature = "libfacedetection")]
    {
        result.push(ruby.to_symbol("libfacedetection"))?;
    }
    #[cfg(feature = "opencv")]
    {
        result.push(ruby.to_symbol("opencv"))?;
    }
    Ok(result)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<()> {
    let module = ruby.define_module("Libfacedetection")?;
    module.define_module_function("detect_opencv", magnus::function!(pub_detect_opencv, 1))?;
    module.define_module_function(
        "detect_libfacedetection",
        magnus::function!(pub_detect_libfacedetection, 1),
    )?;
    module.define_module_function(
        "detect_libfacedetection_image_data",
        magnus::function!(pub_detect_libfacedetection_image_data, 3),
    )?;
    module.define_module_function("features", magnus::function!(features, 0))?;
    Ok(())
}

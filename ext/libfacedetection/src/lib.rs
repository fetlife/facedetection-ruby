use magnus::{error::Result, exception, Error, Integer, RArray, RHash, Symbol};

#[cfg(feature = "opencv")]
fn detect_opencv(content: Vec<u8>) -> Result<RArray> {
    use opencv::{core::Size, imgcodecs, imgproc, objdetect, prelude::*, types};

    let cascade_file_path =
        opencv::core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)
            .map_err(|e| Error::new(exception::runtime_error(), e.to_string()))?;
    let mut classifier = objdetect::CascadeClassifier::new(&cascade_file_path).map_err(|e| {
        Error::new(
            exception::runtime_error(),
            format!("failed creating classifier: {}", e),
        )
    })?;
    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgproc::COLOR_BGR2GRAY)
        .map_err(|e| {
            Error::new(
                exception::runtime_error(),
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
                exception::runtime_error(),
                format!("Failed to run detect_multi_scale: {}", e),
            )
        })?;

    let result = RArray::new();
    for face in faces {
        let hash = RHash::new();
        // let landmarks = RArray::new();
        hash.aset(Symbol::new("x"), Integer::from_i64(face.x as i64))?;
        hash.aset(Symbol::new("y"), Integer::from_i64(face.y as i64))?;
        hash.aset(Symbol::new("width"), Integer::from_i64(face.width as i64))?;
        hash.aset(Symbol::new("height"), Integer::from_i64(face.height as i64))?;
        result.push(hash)?;
    }
    Ok(result)
}

#[cfg(all(feature = "libfacedetection", feature = "opencv"))]
fn detect_libfacedetection(content: Vec<u8>) -> Result<RArray> {
    use opencv::{imgcodecs, prelude::*, types};

    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgcodecs::IMREAD_COLOR)
        .map_err(|e| {
            Error::new(
                exception::runtime_error(),
                format!("unable to decode image: {}", e),
            )
        })?;
    detect_libfacedetection_data(
        img.ptr(0).map_err(|e| {
            Error::new(
                exception::runtime_error(),
                format!("unable to read image data: {}", e),
            )
        })?,
        img.cols(),
        img.rows(),
        Some(img.mat_step().get(0) as u32),
    )
}

#[cfg(not(feature = "opencv"))]
fn detect_opencv(_content: Vec<u8>) -> Result<RArray> {
    Err(Error::new(
        exception::runtime_error(),
        "OpenCV is not enabled",
    ))
}

#[cfg(not(all(feature = "libfacedetection", feature = "opencv")))]
fn detect_libfacedetection(_content: Vec<u8>) -> Result<RArray> {
    Err(Error::new(
        exception::runtime_error(),
        "need to have both libfacedetection and OpenCV enabled",
    ))
}

#[cfg(feature = "libfacedetection")]
fn detect_libfacedetection_data(
    bgrdata: *const u8,
    width: i32,
    height: i32,
    step: Option<u32>,
) -> Result<RArray> {
    let facedetect_result = libfacedetection::facedetect_cnn(
        bgrdata,
        width,
        height,
        step.unwrap_or((width * 3) as u32), // calculate step without padding
    )
    .map_err(|e| {
        Error::new(
            exception::runtime_error(),
            format!("libfacedetection error: {}", e),
        )
    })?;
    let result = RArray::new();
    for face in facedetect_result.faces {
        let hash = RHash::new();
        let landmarks = RArray::new();
        hash.aset(Symbol::new("x"), Integer::from_i64(face.x as i64))?;
        hash.aset(Symbol::new("y"), Integer::from_i64(face.y as i64))?;
        hash.aset(Symbol::new("width"), Integer::from_i64(face.width as i64))?;
        hash.aset(Symbol::new("height"), Integer::from_i64(face.height as i64))?;
        hash.aset(
            Symbol::new("confidence"),
            Integer::from_i64(face.confidence as i64),
        )?;
        for landmark in face.landmarks {
            let a = RArray::new();
            a.push(Integer::from_i64(landmark.0 as i64))?;
            a.push(Integer::from_i64(landmark.1 as i64))?;
            landmarks.push(a)?;
        }
        hash.aset(Symbol::new("landmarks"), landmarks)?;
        result.push(hash)?;
    }
    Ok(result)
}

fn pub_detect_opencv(content: String) -> Result<RArray> {
    detect_opencv(content.into_bytes())
}

fn pub_detect_libfacedetection(content: String) -> Result<RArray> {
    detect_libfacedetection(content.into_bytes())
}

fn pub_detect_libfacedetection_image_data(
    data_ptr: usize,
    width: usize,
    height: usize,
) -> Result<RArray> {
    detect_libfacedetection_data(data_ptr as *const u8, width as i32, height as i32, None)
}

fn features() -> Result<RArray> {
    let result = RArray::new();
    #[cfg(feature = "libfacedetection")]
    {
        result.push(magnus::Symbol::new("libfacedetection"))?;
    }
    #[cfg(feature = "opencv")]
    {
        result.push(magnus::Symbol::new("opencv"))?;
    }
    Ok(result)
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<()> {
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

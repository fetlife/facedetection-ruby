use anyhow::{bail, Result};
use rutie::{methods, module};

use rutie::{Array, Class, Fixnum, Hash, Module, Object, RString, Symbol, VM};

#[cfg(feature = "opencv")]
fn detect_opencv(content: Vec<u8>) -> Result<Array> {
    use opencv::{core::Size, imgcodecs, imgproc, objdetect, prelude::*, types};

    let cascade_file_path =
        opencv::core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?;
    let mut classifier = objdetect::CascadeClassifier::new(&cascade_file_path)
        .context("Unable to open cascade xml file")?;
    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgproc::COLOR_BGR2GRAY)
        .context("Unable to decode image")?;
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
        .context("Failed to run detect_multi_scale")?;

    let mut result = Array::new();
    for face in faces {
        let mut hash = Hash::new();
        let mut landmarks = Array::new();
        hash.store(Symbol::new("x"), Fixnum::new(face.x as i64));
        hash.store(Symbol::new("y"), Fixnum::new(face.y as i64));
        hash.store(Symbol::new("width"), Fixnum::new(face.width as i64));
        hash.store(Symbol::new("height"), Fixnum::new(face.height as i64));
        result.push(hash);
    }
    Ok(result)
}

#[cfg(all(feature = "libfacedetection", feature = "opencv"))]
fn detect_libfacedetection(content: Vec<u8>) -> Result<Array> {
    use opencv::{imgcodecs, prelude::*, types};

    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), imgcodecs::IMREAD_COLOR)
        .context("Unable to decode image")?;
    detect_libfacedetection_data(
        img.ptr(0)?,
        img.cols(),
        img.rows(),
        Some(img.mat_step().get(0) as u32),
    )
}

#[cfg(not(feature = "opencv"))]
fn detect_opencv(_content: Vec<u8>) -> Result<Array> {
    bail!("OpenCV is not enabled");
}

#[cfg(not(all(feature = "libfacedetection", feature = "opencv")))]
fn detect_libfacedetection(_content: Vec<u8>) -> Result<Array> {
    bail!("need to have both libfacedetection and OpenCV enabled");
}

#[cfg(feature = "libfacedetection")]
fn detect_libfacedetection_data(
    bgrdata: *const u8,
    width: i32,
    height: i32,
    step: Option<u32>,
) -> Result<Array> {
    let facedetect_result = libfacedetection::facedetect_cnn(
        bgrdata,
        width,
        height,
        step.unwrap_or((width * 3) as u32), // calculate step without padding
    )?;
    let mut result = Array::new();
    for face in facedetect_result.faces {
        let mut hash = Hash::new();
        let mut landmarks = Array::new();
        hash.store(Symbol::new("x"), Fixnum::new(face.x as i64));
        hash.store(Symbol::new("y"), Fixnum::new(face.y as i64));
        hash.store(Symbol::new("width"), Fixnum::new(face.width as i64));
        hash.store(Symbol::new("height"), Fixnum::new(face.height as i64));
        hash.store(
            Symbol::new("confidence"),
            Fixnum::new(face.confidence as i64),
        );
        for landmark in face.landmarks {
            let mut a = Array::new();
            a.push(Fixnum::new(landmark.0 as i64));
            a.push(Fixnum::new(landmark.1 as i64));
            landmarks.push(a);
        }
        hash.store(Symbol::new("landmarks"), landmarks);
        result.push(hash);
    }
    Ok(result)
}

module!(Libfacedetection);

methods!(
    Libfacedetection,
    _rtself,
    fn pub_detect_opencv(content: RString) -> Array {
        let content = match content {
            Ok(content) => content,
            Err(_) => {
                VM::raise(
                    Class::from_existing("ArgumentError"),
                    "Expected image content",
                );
                unreachable!()
            }
        };
        match detect_opencv(content.to_vec_u8_unchecked()) {
            Ok(faces) => faces,
            Err(e) => {
                VM::raise(Class::from_existing("StandardError"), &format!("{e:#}"));
                unreachable!()
            }
        }
    }

    fn pub_detect_libfacedetection(content: RString) -> Array {
        let content = match content {
            Ok(content) => content,
            Err(_) => {
                VM::raise(
                    Class::from_existing("ArgumentError"),
                    "Expected image content",
                );
                unreachable!()
            }
        };
        match detect_libfacedetection(content.to_vec_u8_unchecked()) {
            Ok(faces) => faces,
            Err(e) => {
                VM::raise(Class::from_existing("StandardError"), &format!("{e:#}"));
                unreachable!()
            }
        }
    }

    fn pub_detect_libfacedetection_image_data(
        data_ptr: Fixnum,
        width: Fixnum,
        height: Fixnum
    ) -> Array {
        let (data_ptr, width, height) = match (data_ptr, width, height) {
            (Ok(data_ptr), Ok(width), Ok(height)) => (data_ptr, width, height),
            _ => {
                VM::raise(
                    Class::from_existing("ArgumentError"),
                    "Provide data_ptr, width and height",
                );
                unreachable!()
            }
        };
        match detect_libfacedetection_data(
            data_ptr.to_u64() as *const u8,
            width.to_i32(),
            height.to_i32(),
            None,
        ) {
            Ok(faces) => faces,
            Err(e) => {
                VM::raise(Class::from_existing("StandardError"), &format!("{e:#}"));
                unreachable!()
            }
        }
    }

    fn features() -> Array {
        let mut result = Array::new();
        #[cfg(feature = "libfacedetection")]
        {
            result.push(Symbol::new("libfacedetection"));
        }
        #[cfg(feature = "opencv")]
        {
            result.push(Symbol::new("opencv"));
        }
        result
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_libfacedetection() {
    Module::new("Libfacedetection").define(|klass| {
        klass.def_self("detect_opencv", pub_detect_opencv);
        klass.def_self("detect_libfacedetection", pub_detect_libfacedetection);
        klass.def_self(
            "detect_libfacedetection_image_data",
            pub_detect_libfacedetection_image_data,
        );
        klass.def_self("features", features);
    });
}

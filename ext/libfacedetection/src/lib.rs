use std::mem;

use anyhow::{bail, Context, Result};
use rutie::{methods, module};

use rutie::{Array, Class, Fixnum, Module, Object, RString, VM};

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
        let mut array = Array::new();
        array.push(Fixnum::new(face.x as i64));
        array.push(Fixnum::new(face.y as i64));
        array.push(Fixnum::new(face.width as i64));
        array.push(Fixnum::new(face.height as i64));
        result.push(array);
    }
    Ok(result)
}

#[cfg(all(feature = "libfacedetection", feature = "opencv"))]
fn detect_libfacedetection(content: Vec<u8>) -> Result<Array> {
    use opencv::{imgcodecs, prelude::*, types};

    let img = imgcodecs::imdecode(&types::VectorOfu8::from(content), 0)
        .context("Unable to decode image")?;
    detect_libfacedetection_data(
        img.ptr(0)?,
        img.cols(),
        img.rows(),
        Some(img.mat_step().get(0) as u32),
    )
}

#[cfg(not(feature = "opencv"))]
fn detect_opencv(content: Vec<u8>) -> Result<Array> {
    bail!("OpenCV is not enabled");
}

#[cfg(not(all(feature = "libfacedetection", feature = "opencv")))]
fn detect_libfacedetection(content: Vec<u8>) -> Result<Array> {
    bail!("need to have both libfacedetection and OpenCV enabled");
}

#[cfg(feature = "libfacedetection")]
fn detect_libfacedetection_data(
    brgdata: *const u8,
    width: i32,
    height: i32,
    step: Option<u32>,
) -> Result<Array> {
    let facedetect_result = libfacedetection::facedetect_cnn(
        brgdata,
        width,
        height,
        step.unwrap_or_else(|| (width * 3) as u32), // calculate step without padding
    )?;
    let mut result = Array::new();
    for face in facedetect_result.faces {
        let mut array = Array::new();
        array.push(Fixnum::new(face.x as i64));
        array.push(Fixnum::new(face.y as i64));
        array.push(Fixnum::new(face.width as i64));
        array.push(Fixnum::new(face.height as i64));
        result.push(array);
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

    fn pub_detect_libfacedetection_image_data(data_ptr: Fixnum, width: Fixnum, height: Fixnum) -> Array {
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
        match unsafe {
            detect_libfacedetection_data(
                mem::transmute(data_ptr.to_u64()),
                width.to_i32(),
                height.to_i32(),
                None,
            )
        } {
            Ok(faces) => faces,
            Err(e) => {
                VM::raise(Class::from_existing("StandardError"), &format!("{e:#}"));
                unreachable!()
            }
        }
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
    });
}

use anyhow::{Context, Result};
use opencv;
use rutie::{methods, module};

use opencv::core::{self, Size};
use rutie::{Array, Class, Fixnum, Module, Object, RString, VM};

use opencv::{imgcodecs, imgproc, objdetect, prelude::*, types};

fn detect_haar_cascade(content: Vec<u8>) -> Result<Array> {
    let cascade_file_path =
        core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?;
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
            Size::new(100, 100),
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

fn do_detection(content: Vec<u8>) -> Result<Array> {
    detect_haar_cascade(content)
}

module!(Libfacedetection);

methods!(
    Libfacedetection,
    rtself,
    fn pub_detect(content: RString) -> Array {
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
        match do_detection(content.to_vec_u8_unchecked()) {
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
        klass.def_self("detect", pub_detect);
    });
}

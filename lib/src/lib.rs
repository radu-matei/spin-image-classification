#[allow(warnings)]
mod bindings;

use bindings::exports::component::image_classification_lib::image_classification::ClassificationResult;
use bindings::exports::component::image_classification_lib::image_classification::{
    ClassificationError, Image,
};
use std::io::BufRead;
use std::io::Cursor;
use tract_tensorflow::prelude::*;

use crate::bindings::exports::component::image_classification_lib::image_classification::Guest;

struct Component;

impl Guest for Component {
    fn classify(img: Image) -> Result<ClassificationResult, ClassificationError> {
        let model = tract_tensorflow::tensorflow()
            .model_for_read(&mut Cursor::new(include_bytes!(
                "../mobilenet_v2_1.4_224_frozen.pb"
            )))?
            .with_input_fact(0, f32::fact([1, 224, 224, 3]).into())?
            .into_optimized()?
            .into_runnable()?;

        println!("[Rust classifier]: Loaded Tensorflow model.");

        let image = image::load_from_memory(&img)?.to_rgb8();
        let resized =
            image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
        let image: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
                resized[(x as _, y as _)][c] as f32 / 255.0
            })
            .into();

        println!("[Rust classifier]: Resized image to 224x224 px.");

        // run the model on the input
        let result = model.run(tvec!(image.into()))?;
        // find and display the max value with its index
        let best = result[0]
            .to_array_view::<f32>()?
            .iter()
            .cloned()
            .zip(1..)
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        match best {
            Some((probability, class)) => {
                let label = get_label(class)?;
                println!(
                    "[Rust classifier]: Probability: {}, class: {}.",
                    probability, label
                );
                return Ok((label, probability));
            }
            None => return Err(ClassificationError::Unclassified),
        }
    }
}

bindings::export!(Component with_types_in bindings);

impl From<TractError> for ClassificationError {
    fn from(e: TractError) -> Self {
        ClassificationError::ModelError(e.to_string())
    }
}

impl From<image::ImageError> for ClassificationError {
    fn from(e: image::ImageError) -> Self {
        ClassificationError::ImageError(e.to_string())
    }
}

fn get_label(num: usize) -> Result<String, anyhow::Error> {
    // The result of executing the inference is the predicted class,
    // which also indicates the line number in the (1-indexed) labels file.
    let labels = include_bytes!("../labels.txt");
    let content = std::io::BufReader::new(Cursor::new(labels));
    content
        .lines()
        .nth(num - 1)
        .expect("cannot get prediction label")
        .map_err(|err| anyhow::Error::new(err))
}

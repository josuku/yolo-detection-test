use crate::{ort_detector::OrtDetector, ultralytics_detector::UltralyticsDetector};

mod ort_detector;
mod ultralytics_detector;

fn main() -> anyhow::Result<()> {
    let model_path = "./models/yolo11n.onnx";
    let image_path = "images/test.jpg";

    let mut detector = OrtDetector::new(model_path).expect("detector not initialized");
    match detector.detect(image_path) {
        Ok(_) => (),
        Err(err) => log::error!("{err}"),
    }

    println!("");

    let mut ultraltyics_detector =
        UltralyticsDetector::new(model_path).expect("detector not initialized");
    match ultraltyics_detector.detect(image_path) {
        Ok(_) => (),
        Err(err) => log::error!("{err}"),
    }

    Ok(())
}

use ultralytics_inference::YOLOModel;

pub struct UltralyticsDetector {
    model: YOLOModel,
}

impl UltralyticsDetector {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        println!("UltralyticsDetector - Loading model...");
        let model = ultralytics_inference::YOLOModel::load(model_path)?;

        Ok(Self { model })
    }

    pub fn detect(&mut self, image_path: &str) -> anyhow::Result<()> {
        println!("UltralyticsDetector - Loading image ...");

        let results = self.model.predict(image_path)?;

        for result in &results {
            if let Some(boxes) = &result.boxes {
                for i in 0..boxes.len() {
                    let class_id = boxes.cls()[i] as usize;
                    let conf = boxes.conf()[i];
                    let name = result
                        .names
                        .get(&class_id)
                        .map_or("unknown", |s| s.as_str());
                    println!("UltralyticsDetector - {name} {conf:.2}");
                }
            }
        }

        Ok(())
    }
}

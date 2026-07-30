use image::imageops::FilterType;
use ort::{session::Session, value::TensorRef};

pub struct OrtDetector {
    session: Session,
}

impl OrtDetector {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        println!("OrtDetector - Loading model...");

        let session = Session::builder()?.commit_from_file(model_path)?;

        Ok(Self { session })
    }

    pub fn detect(&mut self, image_path: &str) -> anyhow::Result<()> {
        println!("OrtDetector - Loading image ...");

        let img = image::open(image_path)?
            .resize_exact(640, 640, FilterType::Triangle)
            .to_rgb8();

        println!("OrtDetector - Creating tensor...");

        let mut input = vec![0f32; 3 * 640 * 640];

        println!("OrtDetector - Tensor allocated");

        for (x, y, pixel) in img.enumerate_pixels() {
            let idx = y as usize * 640 + x as usize;

            input[idx] = pixel[0] as f32 / 255.0;
            input[640 * 640 + idx] = pixel[1] as f32 / 255.0;
            input[2 * 640 * 640 + idx] = pixel[2] as f32 / 255.0;
        }

        println!("OrtDetector - Tensor filled");

        println!("OrtDetector - Running inference...");

        let shape = [1, 3, 640, 640];

        let data = input.as_slice(); //.unwrap();

        let outputs = self
            .session
            .run(ort::inputs![TensorRef::from_array_view((shape, data))?])?;

        println!("OrtDetector - Number of outputs: {}", outputs.len());

        println!(
            "OrtDetector - First values: {:?}",
            &data[..20.min(data.len())]
        );
        Ok(())
    }
}

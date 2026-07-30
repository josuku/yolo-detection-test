# Yolo Detection Test

Example made with ort and with ultralytics_detector crate to detect objects in pictures

Ultralytics detector needs onnx runtime libs to work. Runtime needed files (1.28.0) are loaded from runtime folder on build stage (build.rs) and downloaded from:
https://github.com/microsoft/onnxruntime/releases

### Run

``` cargo run```

### Linux Requirements:
This packages installation is required:

``` sudo apt install -y build-essential libssl-dev pkg-config```


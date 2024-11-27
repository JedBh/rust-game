use onnxruntime::{environment::Environment, tensor::OrtOwnedTensor, GraphOptimizationLevel};

fn main() {
    let env = Environment::builder().with_name("test").build().unwrap();
    let session = env
        .new_session_builder()
        .unwrap()
        .with_optimization_level(GraphOptimizationLevel::Basic)
        .unwrap()
        .with_model_from_file("yolo.onnx")
        .unwrap();

    // Load and preprocess the image
    let input_tensor: Vec<f32> = preprocess_image("dog.jpg"); // Custom function

    // Run the model
    let outputs: Vec<OrtOwnedTensor<f32>> = session.run(vec![input_tensor.into()]).unwrap();

    // Parse and print predictions
    for prediction in parse_yolo_outputs(outputs) {
        // Custom function
        println!("Detected object: {:?}", prediction);
    }
}

//! examples/cube
//!
//! This example is a simple cube.
//!

fn main() {
    let vertices: Vec<(f32, f32, f32)> = vec![
        (0.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (1.0, 0.0, 1.0),
        (0.0, 1.0, 1.0),
        (1.0, 1.0, 1.0),
    ];

    println!("vertex count: {}", vertices.len());
}
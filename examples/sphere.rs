//! examples/sphere
//!
//! This example is a simple sphere.
//!

fn main() {
    let points = fibonacci_sphere(100);

    println!("{} points", points.len());
}

// Based upon: https://stackoverflow.com/a/26127012
fn fibonacci_sphere(n: u32) -> Vec<(f32, f32, f32)> {
    let mut points = Vec::new();

    let phi = std::f32::consts::PI * (5.0_f32.sqrt() - 1.0);

    for i in 0..n {
        let y = 1.0 - (i as f32 / (n as f32 - 1.0)) * 2.0;
        let radius = (1.0 - y.powf(2.0)).sqrt();

        let theta = phi * i as f32;

        let x = theta.cos() * radius;
        let z = theta.sin() * radius;

        points.push((x, y, z));
    }

    points
}
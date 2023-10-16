//! examples/bathtub
//!
//! This example dataset is of a bathtub obtained from Princeton's ModelNet project.
//! https://modelnet.cs.princeton.edu
//!

use crate::off::read_off_file_vertices;

mod off;

fn main() {
    let vertices = read_off_file_vertices("examples/bathtub_0106.off")
        .expect("can decode OFF file");

    println!("vertex count: {}", vertices.len());
}
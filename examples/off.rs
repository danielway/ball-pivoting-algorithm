//! Utilities for reading OFF object files. https://en.wikipedia.org/wiki/OFF_(file_format)

/// Reads and decodes an OFF object file's vertices, ignoring edges/faces.
pub fn read_off_file_vertices(path: &str) -> std::io::Result<Vec<(f32, f32, f32)>> {
    let mut vertices = Vec::new();

    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();

    // Skip the first line, which is just "OFF"
    lines.next();

    // Read the second line, which contains the number of vertices, edges, and faces
    let line = lines.next().unwrap()?;
    let mut line = line.split_whitespace();
    let num_vertices: usize = line.next().unwrap().parse().unwrap();

    // Read the vertices
    for _ in 0..num_vertices {
        let line = lines.next().unwrap()?;
        let mut line = line.split_whitespace();
        let x: f32 = line.next().unwrap().parse().unwrap();
        let y: f32 = line.next().unwrap().parse().unwrap();
        let z: f32 = line.next().unwrap().parse().unwrap();
        vertices.push((x, y, z));
    }

    Ok(vertices)
}

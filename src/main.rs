extern crate nalgebra as na;
extern crate ncollide2d;

use na::Point2;
use ncollide2d::shape::TriMesh;

fn main() {
    // Define the vertices.
    let vertices = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ];


    // Define the triangles.
    let triangles = vec![
        Point2::new(0usize, 1usize),  // First edge of the triangle.
        Point2::new(1usize, 2usize),  // Second edge of the triangle.
        Point2::new(2usize, 0usize),  // Third edge of the triangle.
    ];

    // Create the triangle mesh.
    let tri_mesh = TriMesh::new(vertices, triangles, None, None);

    println!("{:?}", tri_mesh);  // Display the created
}
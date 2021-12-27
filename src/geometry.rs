use glium::backend::Facade;
use glium::Display;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

pub fn create_quad<F: ?Sized>(display: &F) -> glium::vertex::VertexBuffer<Vertex> where F: Facade {
    return glium::vertex::VertexBuffer::new(display, &[
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ]).unwrap();
}
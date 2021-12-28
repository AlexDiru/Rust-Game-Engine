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

    let v_vec = create_cube_vertexes(nalgebra_glm::Vec3::new(0.0, 0.0, 0.0));

    return glium::vertex::VertexBuffer::new(
        display,
        v_vec.as_slice()
    ).unwrap();
}

fn create_cube_vertexes(position: nalgebra_glm::Vec3) -> Vec<Vertex> {

    let cube_vertices: [i32;180] = [
        // Vert x, vert y, vert z, tex x, tex y

        // FRONT FACE
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 1,
        1, 0, 0, 1, 0,

        1, 0, 0, 1, 0,
        0, 1, 0, 0, 1,
        1, 1, 0, 1, 1,

        // RIGHT FACE
        1, 1, 0, 0, 1,
        1, 0, 1, 1, 0,
        1, 0, 0, 0, 0,

        1, 1, 0, 0, 1,
        1, 0, 1, 1, 0,
        1, 1, 1, 1, 1,

        // TOP FACE
        1, 1, 1, 1, 1,
        1, 1, 0, 1, 0,
        0, 1, 1, 0, 1,

        1, 1, 0, 1, 0,
        0, 1, 1, 0, 1,
        0, 1, 0, 0, 0,

        // LEFT FACE
        0, 1, 0, 1, 1,
        0, 0, 0, 1, 0,
        0, 1, 1, 0, 1,

        0, 1, 1, 0, 1,
        0, 0, 0, 1, 0,
        0, 0, 1, 0, 0,

        // BACKN FACE
        0, 0, 1, 1, 0,
        0, 1, 1, 1, 1,
        1, 1, 1, 0, 1,

        1, 1, 1, 0, 1,
        0, 0, 1, 1, 0,
        1, 0, 1, 0, 0,

        // BOTTOM FACE
        1, 0, 1, 1, 1,
        1, 0, 0, 1, 0,
        0, 0, 1, 0, 1,

        0, 0, 1, 0, 1,
        1, 0, 0, 1, 0,
        0, 0, 0, 0, 0,
    ];

    let mut vs = Vec::new();

    for i in 0..(cube_vertices.len()/5) {
        let v = i * 5;
        vs.push(Vertex {
            position: [ cube_vertices[v] as f32, cube_vertices[v + 1] as f32, cube_vertices[v + 2] as f32 ],
            normal: [0.0, 0.0, 0.0 ],
            tex_coords: [ cube_vertices[v + 3] as f32, cube_vertices[v + 4] as f32],
        })
    }

    vs
}

fn create_quad_vertexes(position: nalgebra_glm::Vec3) -> [Vertex; 4] {
    return [
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ];
}
use glium::{Frame};
use glium::backend::Facade;
use crate::geometry::{create_cube, create_cube_vertexes, create_vertex_buffer, Vertex};

pub struct Cube {
    mat: nalgebra_glm::Mat4,
    position: nalgebra_glm::Vec3,
    vertexes: Vec<Vertex>,
    _local_center: nalgebra_glm::Vec3
}

impl Cube {
    pub fn new<F: ?Sized>(display: &F) -> Cube where F: Facade {
        let mut mat = nalgebra_glm::Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0f32
        );

        Cube {
            mat,
            vertexes: create_cube_vertexes(),
            position: nalgebra_glm::Vec3::new(0.0, 0.0, 0.0),
            _local_center: nalgebra_glm::Vec3::new(0.5, 0.5, 0.5)
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        let new_position = nalgebra_glm::Vec3::new(x, y, z);
        let translation = new_position - self.position;

        self.position = new_position;
        self.mat = nalgebra_glm::translate(&self.mat, &translation);
    }

    pub fn get_position(&self) -> [f32; 3] {
        [
            self.position.x,
            self.position.y,
            self.position.z
        ]
    }

    pub fn get_position_vec(&self) -> &nalgebra_glm::Vec3 {
        &self.position
    }

    pub fn rotate(&mut self, angleRad: f32, x: f32, y: f32, z: f32) {

        if x == 0.0 && y == 0.0 && z == 0.0 {
            return
        }

        let vec = nalgebra_glm::vec3(x, y, z);


        let rotation_vec = vec.normalize();

        // Translate back to 0,0,0
        // Rotate
        // Translate back to position
        self.mat = nalgebra_glm::translate(&self.mat, &(self._local_center - self.position));

        self.mat = nalgebra_glm::rotate(&self.mat, angleRad, &rotation_vec);
        self.mat = nalgebra_glm::translate(&self.mat, &(self.position - self._local_center));
    }

    pub fn get_mat(&self) -> [[f32; 4]; 4] {
        return self.mat.data.0
    }

    pub fn create_vertex_buffer<F: ?Sized>(&self, display: &F) -> glium::vertex::VertexBuffer<Vertex> where F: Facade {
        create_vertex_buffer(display, &self.vertexes)
    }

    // fn get_tri(&self, i: i32) -> [VertexBuffer<Vertex>; 3] {
    //     n = i * 3
    //     [
    //         self.vertex_buffer
    //     ]
    // }
    //
    // pub fn get_tris(&self) -> [VertexBuffer<Vertex>; 12] {
    // }
}
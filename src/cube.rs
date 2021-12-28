pub struct Cube {
    mat: nalgebra_glm::Mat4
}

impl Cube {
    pub fn new() -> Cube {
        let mut mat = nalgebra_glm::Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0f32
        );

        Cube {
            mat
        }
    }

    pub fn set_rotation(&mut self, angleRad: f32, x: f32, y: f32, z: f32) {
        let rotation_vec = nalgebra_glm::vec3(x, y, z).normalize();
        self.mat = nalgebra_glm::rotate(&self.mat, angleRad, &rotation_vec)
    }

    pub fn get_mat(&self) -> [[f32; 4]; 4] {
        return self.mat.data.0
    }
}
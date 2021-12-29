use std::f32::consts::PI;

pub(crate) struct Camera {
    pub position: nalgebra_glm::Vec3,
    pub rotation: nalgebra_glm::Vec3
}

impl Camera {
    pub fn get_forward_direction(&self) -> nalgebra_glm::Vec3 {
        compute_direction(self.rotation.x, self.rotation.y)
    }

    pub fn get_left_direction(&self) -> nalgebra_glm::Vec3 {
        compute_direction(self.rotation.x, self.rotation.y + (PI/2.0))
    }
}

fn compute_direction(x_angle: f32, y_angle: f32) -> nalgebra_glm::Vec3{
    nalgebra_glm::Vec3::new(
        y_angle.cos() * x_angle.cos(),
        x_angle.sin(),
        y_angle.sin() * x_angle.cos())
}
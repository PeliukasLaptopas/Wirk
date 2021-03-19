// use crate::na::Vector2;
// use crate::na::Matrix4;
use nalgebra::{one, Point3, Isometry3};
use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;

//If window is resized this needs to be reinitialized to adjust data!
pub struct Camera2D {
    pub position: Vector2<f32>,
    pub camera_matrix: Matrix4<f32>,
    pub ortho_matrix: Matrix4<f32>,
    pub scale: f32,
    needs_matrix_update: bool,
    //we need to tell camera what are viewport dimensions are
    screen_width: u32,
    screen_height: u32,
}

impl Camera2D {
    pub fn new(pos: Vector2<f32>, scale: f32, screen_width: u32, screen_height: u32) -> Camera2D {
        Camera2D {
            position: pos,
            camera_matrix: one::<Matrix4<f32>>(),
            ortho_matrix: Matrix4::new_orthographic(
                0.0,
                screen_width as f32,
                0.0,
                screen_height as f32,
                -10000.0,
                10000.0,
            ),
            scale,
            needs_matrix_update: true,
            screen_width,
            screen_height,
        }
    }

    pub fn set_position(&mut self, new_position: Vector2<f32>) {
        self.position = new_position;
        self.needs_matrix_update = true;
    }

    pub fn set_scale(&mut self, new_scale: f32) {
        self.scale = new_scale;
        self.needs_matrix_update = true;
    }

    pub fn update(&mut self) {
        if (self.needs_matrix_update) {
            let translate = Vector3::new(-self.position.x, -self.position.y, 0.0);
            self.camera_matrix = nalgebra_glm::translate(&self.ortho_matrix, &translate);
            let scale = Vector3::new(self.scale, self.scale, 0.0);
            self.camera_matrix = nalgebra_glm::scale(&self.camera_matrix, &scale);
            self.needs_matrix_update = false;
        }
    }
}

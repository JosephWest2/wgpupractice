use nalgebra::{Point3, Vector3, Matrix4};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = nalgebra::Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj = nalgebra::Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj.as_matrix() * view;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(camera: &Camera) -> Self {
        Self {
            view_proj: camera.build_view_projection_matrix().into(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[derive(Debug)]
pub struct CameraController {
    pub speed: f32,
    pub forward_pressed: bool,
    pub backward_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,

        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_normalized = forward.normalize();

        if (self.forward_pressed) {
            camera.eye += forward_normalized * self.speed;
        }
        if (self.backward_pressed) {
            camera.eye -= forward_normalized * self.speed;
        }
    }
}


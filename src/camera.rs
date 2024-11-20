use nalgebra::{Matrix4, Point3, Vector3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

type Rad = f32;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub pitch: Rad,
    pub yaw: Rad,
    pub up: Vector3<f32>,
    // screen width over screen height
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Point3::new(0.0, 1.0, 2.0),
            pitch: 0.0,
            yaw: 0.0,
            up: Vector3::y_axis().into_inner(),
            aspect,
            fovy: 45.0,
            znear: 0.01,
            zfar: 100.0,
        }
    }
    pub fn get_camera_forward(&self) -> Vector3<f32> {
        // Defaults to unit vector in the -Z axis
        Vector3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            -1.0 * self.pitch.cos() * self.yaw.cos(),
        )
    }
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = nalgebra::Matrix4::look_at_rh(
            &self.position,
            &(self.position + self.get_camera_forward()),
            &self.up,
        );
        let proj = nalgebra::Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj.as_matrix() * view;
    }
    pub fn get_uniform(&self) -> CameraUniform {
        CameraUniform {
            view_proj: self.build_view_projection_matrix().into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

#[derive(Debug)]
pub struct CameraController {
    pub speed: f32,
    pub mouse_sens: f32,
    pub mouse_delta: (f32, f32),
    pub forward_pressed: bool,
    pub backward_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32, mouse_sens: f32) -> Self {
        Self {
            speed,
            mouse_sens,
            mouse_delta: (0.0, 0.0),
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let forward = camera.get_camera_forward();
        let right = forward.cross(&Vector3::y_axis());
        if self.forward_pressed {
            camera.position += forward * self.speed;
        }
        if self.backward_pressed {
            camera.position -= forward * self.speed;
        }
        if self.left_pressed {
            camera.position -= right * self.speed;
        }
        if self.right_pressed {
            camera.position += right * self.speed;
        }
        camera.yaw += self.mouse_delta.0 * self.mouse_sens;
        camera.pitch -= self.mouse_delta.1 * self.mouse_sens;
        self.mouse_delta = (0.0, 0.0);
    }
}

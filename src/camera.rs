use nalgebra::{Point3, Matrix4};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

type Rad = f32;
struct Camera {
    position: Point3<f32>,
    pitch: Rad,
    yaw: Rad,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = nalgebra::Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj = nalgebra::Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * view * proj.as_matrix();
    }
}

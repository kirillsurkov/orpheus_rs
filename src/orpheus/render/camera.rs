use super::*;

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    transform: [[f32; 4]; 4],
}

impl Camera {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;

        Self {
            eye: (0.0, 0.0, 0.0).into(),
            target: (0.0, 0.0, 1.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
            transform: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        self.transform = (misc::OPENGL_TO_WGPU_MATRIX * proj * view).into();
    }

    pub fn transform(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }

    pub fn look_at(&mut self, eye: [f32; 3], target: [f32; 3]) {
        self.eye = eye.into();
        self.target = target.into();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

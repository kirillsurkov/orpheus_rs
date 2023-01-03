use super::misc;

struct Data {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

pub struct Camera {
    data: Data,
    transform: misc::UniformBuffer<misc::Transform>,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        use cgmath::SquareMatrix;

        let transform = misc::UniformBuffer::new(
            device,
            "camera_transform",
            misc::Transform {
                matrix: cgmath::Matrix4::identity().into(),
            },
        );

        Self {
            data: Data {
                eye: (0.0, 0.0, 0.0).into(),
                target: (0.0, 0.0, 1.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: 1.0,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            transform,
        }
    }

    pub fn update_view_proj(&mut self) {
        let view = cgmath::Matrix4::look_at_rh(self.data.eye, self.data.target, self.data.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.data.fovy),
            self.data.aspect,
            self.data.znear,
            self.data.zfar,
        );
        self.transform.data().matrix = (misc::OPENGL_TO_WGPU_MATRIX * proj * view).into();
    }

    pub fn look_at(&mut self, eye: &cgmath::Point3<f32>, target: &cgmath::Point3<f32>) {
        self.data.eye = *eye;
        self.data.target = *target;
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.data.aspect = aspect;
    }

    pub fn transform(&self) -> &misc::UniformBuffer<misc::Transform> {
        &self.transform
    }
}

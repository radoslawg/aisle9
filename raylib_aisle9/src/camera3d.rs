use crate::vector::Vector3;

pub struct Camera3D {
    pub camera: raylib_ffi::Camera3D,
}

pub enum CameraProjection {
    CameraPerspective = 0,
    CameraOrthographic = 1,
}

impl Camera3D {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fovy: f32,
        projection: CameraProjection,
    ) -> Camera3D {
        Camera3D {
            camera: raylib_ffi::Camera3D {
                position: position.raw(),
                target: target.raw(),
                up: up.raw(),
                fovy,
                projection: projection as i32,
            },
        }
    }
    pub fn update(&mut self) {
        unsafe {
            raylib_ffi::UpdateCamera(&mut self.camera, 0);
        }
    }
}

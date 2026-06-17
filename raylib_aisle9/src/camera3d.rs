use crate::vector::Vector3;

pub struct Camera3D {
    pub camera: raylib_ffi::Camera3D,
}

impl Camera3D {
    pub fn new(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        Camera3D {
            camera: raylib_ffi::Camera3D {
                position: position.raw(),
                target: target.raw(),
                up: up.raw(),
                fovy,
                projection: 0,
            },
        }
    }
    pub fn update(&mut self) {
        unsafe {
            raylib_ffi::UpdateCamera(&mut self.camera, 0);
        }
    }
}

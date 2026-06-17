use std::ops::Add;

pub type Vector2 = raylib_ffi::Vector2;
pub type Vector4 = raylib_ffi::Vector4;
pub type Quaternion = raylib_ffi::Quaternion;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn origin() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub(crate) fn from_raw(raw: raylib_ffi::Vector3) -> Vector3 {
        Vector3 {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        }
    }
    pub(crate) fn raw(&self) -> raylib_ffi::Vector3 {
        raylib_ffi::Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

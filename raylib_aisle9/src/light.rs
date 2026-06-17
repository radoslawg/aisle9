use crate::{
    Color,
    vector::{Quaternion, Vector3},
};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Vector3,
    pub falloff: f32,
    pub color: Color,
    pub rotation: Option<Quaternion>,
    pub direction: Option<Vector3>,
}

impl Light {
    pub fn new(position: Vector3, falloff: f32, color: Color) -> Self {
        Light {
            position,
            falloff,
            color,
            rotation: None,
            direction: None,
        }
    }

    pub fn with_rotation(&mut self, rotation: Quaternion) -> Self {
        self.rotation = Some(rotation);
        let gltf_forward = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        unsafe {
            let normalized_rotation = raylib_ffi::Vector4Normalize(rotation);
            self.direction = Some(Vector3::from_raw(raylib_ffi::Vector3Normalize(
                raylib_ffi::Vector3RotateByQuaternion(gltf_forward.raw(), normalized_rotation),
            )));
        }
        *self
    }

    pub fn enabled(&self, target: Vector3) -> bool {
        f32::sqrt(
            f32::powf(target.x - self.position.x, 2.0)
                + f32::powf(target.y - self.position.y, 2.0)
                + f32::powf(target.z - self.position.z, 2.0),
        ) <= self.falloff
    }
    pub fn load_lights(file_path: &str) -> Vec<Self> {
        log::trace!("Trying to load lights from {file_path}");
        let Ok((document, _, _)) = gltf::import(file_path) else {
            return Vec::new();
        };
        let mut result: Vec<Light> = Vec::with_capacity(4);
        for node in document.nodes() {
            if let Some(light) = node.light() {
                let light_color = light.color();
                let (translation, rotation, _scale) = node.transform().decomposed();
                let new_light = Light {
                    falloff: light.range().unwrap_or(20.0),
                    color: Color {
                        r: (light_color[0] * 255.0) as u8,
                        g: (light_color[1] * 255.0) as u8,
                        b: (light_color[2] * 255.0) as u8,
                        a: 255,
                    },
                    position: Vector3::from_raw(raylib_ffi::Vector3 {
                        x: translation[0],
                        y: translation[1],
                        z: translation[2],
                    }),
                    rotation: None,
                    direction: None,
                }
                .with_rotation(Quaternion {
                    x: rotation[0],
                    y: rotation[1],
                    z: rotation[2],
                    w: rotation[3],
                });
                log::trace!("Loaded light from {} with data: {:?}", file_path, new_light);
                result.push(new_light);
            }
        }
        result
    }
}

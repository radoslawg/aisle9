use crate::{
    colors::{self},
    light::Light,
    material::Material,
    shader::Shader,
    vector::Vector3,
};
use std::ffi::CString;

pub struct Model {
    model: raylib_ffi::Model,
    position: Vector3,
    raw_lights: Vec<Light>,
}

impl Model {
    pub fn load_model(path: &str) -> Model {
        unsafe {
            let model: raylib_ffi::Model =
                raylib_ffi::LoadModel(CString::new(path).unwrap().as_ptr());
            let raw_lights = Light::load_lights(path);
            Model {
                model,
                raw_lights,
                position: Vector3::origin(),
            }
        }
    }

    pub fn get_lights(&self) -> Vec<Light> {
        let mut transformed_lights = Vec::with_capacity(self.raw_lights.len());
        for light in &self.raw_lights {
            transformed_lights.push(Light {
                position: light.position + self.position,
                ..*light
            });
        }
        transformed_lights
    }

    pub fn with_position(&mut self, position: Vector3) -> &Self {
        self.position = position;
        self
    }

    pub fn get_material(&mut self, index: usize) -> Option<Material<'_>> {
        if index >= self.model.materialCount as usize {
            return None;
        }
        unsafe {
            let mat_ptr = self.model.materials.add(index);
            log::trace!("Material found: {}, {:?}", index, mat_ptr);
            Some(Material::from_raw_mut(mat_ptr))
        }
    }
    pub fn draw_model(&self) {
        unsafe { raylib_ffi::DrawModel(self.model, self.position.raw(), 1.0, colors::WHITE) }
    }

    pub fn set_shader(&self, shader: &Shader) {
        log::trace!("Shader set.");
        unsafe {
            let materials = std::slice::from_raw_parts_mut(
                self.model.materials,
                self.model.materialCount as usize,
            );
            for material in materials {
                material.shader = shader.shader;
            }
        }
    }
}

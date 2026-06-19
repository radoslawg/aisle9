use crate::{
    colors::{self},
    light::Light,
    material::Material,
    shader::Shader,
    vector::Vector3,
};
use std::{ffi::CString, path::Path};

pub struct Model {
    pub model: raylib_ffi::Model,
    pub raw_lights: Vec<Light>,
}

impl Model {
    pub fn load_model(path: &Path) -> Model {
        unsafe {
            let model: raylib_ffi::Model =
                raylib_ffi::LoadModel(CString::new(path.to_str().unwrap()).unwrap().as_ptr());
            let raw_lights = Light::load_lights(path);
            Model { model, raw_lights }
        }
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
}

pub struct WorldModel<'a> {
    model: &'a Model,
    position: Vector3,
}

// Safety: raylib is single-threaded; Model is only created and used on the
// main (OpenGL context) thread. The raw C pointers it wraps are not accessed
// from any other thread.
unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl<'a> WorldModel<'a> {
    pub fn new(model: &'a Model) -> Self {
        WorldModel {
            model,
            position: Vector3::origin(),
        }
    }
    pub fn get_lights(&self) -> Vec<Light> {
        let mut transformed_lights = Vec::with_capacity(self.model.raw_lights.len());
        for light in &self.model.raw_lights {
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

    pub fn draw_model(&self) {
        unsafe { raylib_ffi::DrawModel(self.model.model, self.position.raw(), 1.0, colors::WHITE) }
    }

    pub fn set_shader(&self, shader: &Shader) {
        log::trace!("Shader set.");
        unsafe {
            let materials = std::slice::from_raw_parts_mut(
                self.model.model.materials,
                self.model.model.materialCount as usize,
            );
            for material in materials {
                material.shader = shader.shader;
            }
        }
    }
}

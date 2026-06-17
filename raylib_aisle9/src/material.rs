use crate::texture2d::Texture2D;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum MaterialMapIndex {
    Albedo = 0,
    Metalness = 1,
    Normal = 2,
    Roughness = 3,
    Occlusion = 4,
    Emission = 5,
    Height = 6,
    Cubemap = 7,
    Irradiance = 8,
    Prefilter = 9,
    Brdf = 10,
}

pub struct Material<'a> {
    raw: &'a mut raylib_ffi::Material,
}

impl<'a> Material<'a> {
    /// # Safety
    /// 'ptr' must be valid (duh)
    pub unsafe fn from_raw_mut(ptr: *mut raylib_ffi::Material) -> Self {
        unsafe { Self { raw: &mut *ptr } }
    }

    /// Change the underlying texture for a specific map
    pub fn set_texture(&mut self, map_type: MaterialMapIndex, texture: Texture2D) {
        // Raylib hardcodes MAX_MATERIAL_MAPS to 12 in raylib.h
        const MAX_MATERIAL_MAPS: usize = 12;

        unsafe {
            let maps_slice = std::slice::from_raw_parts_mut(self.raw.maps, MAX_MATERIAL_MAPS);

            log::trace!("Before Texture Set: {:?}", maps_slice[map_type as usize]);
            maps_slice[map_type as usize].texture = texture;
            log::trace!("After Texture Set: {:?}", maps_slice[map_type as usize]);
        }
        log::trace!("Texture for {:?} set.", map_type);
    }

    pub fn get_map_id(&self, map_type: MaterialMapIndex) -> u32 {
        const MAX_MATERIAL_MAPS: usize = 12;
        unsafe {
            let maps_slice = std::slice::from_raw_parts_mut(self.raw.maps, MAX_MATERIAL_MAPS);
            maps_slice[map_type as usize].texture.id
        }
    }
}

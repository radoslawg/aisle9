use std::{collections::HashMap, sync::OnceLock};

use raylib_aisle9::texture2d::Texture2D;

pub static ASSET_MANAGER: OnceLock<AssetManager> = OnceLock::new();

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub fn init() {
        let mut testeroni = HashMap::new();
        ASSET_MANAGER
            .set(AssetManager {
                textures: testeroni,
            })
            .is_err()
            .then(|| {
                panic!("AssetManager has already been initialized");
            });
    }

    pub fn get() -> &'static AssetManager {
        ASSET_MANAGER
            .get()
            .expect("AssetManager is not initialized")
    }

    fn load_asset(&self, asset_path: &str) {
        // Load the asset from the given path
        println!("Loading asset from: {}", asset_path);
    }

    fn unload_asset(&self, asset_path: &str) {
        // Unload the asset from memory
        println!("Unloading asset from: {}", asset_path);
    }
}

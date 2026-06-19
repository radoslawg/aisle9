use std::{collections::HashMap, path::Path, sync::OnceLock};

use raylib_aisle9::{load_texture, model::Model, texture2d::Texture2D};

use crate::assets_config::Assets;

pub static ASSET_MANAGER: OnceLock<AssetManager> = OnceLock::new();

pub struct AssetManager {
    pub models: HashMap<String, Model>,
    pub textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub fn init() {
        let hjson_text = std::fs::read_to_string("assets/assets.hjson")
            .expect("failed to read assets.hjson in AssetManager::init");
        let assets: Assets =
            deser_hjson::from_str(&hjson_text).expect("failed to parse assets.hjson");

        println!(
            "Loading textures from assets.hjson... {:?}",
            assets.textures
        );
        let mut textures = HashMap::new();
        let file_name = &&assets.textures.get("Grid").unwrap().file;
        let texture_path = Path::new("assets/textures").join(file_name);
        let texture = load_texture(texture_path.as_path());
        textures.insert(String::from("grid"), texture);

        let mut models = HashMap::new();

        println!("Loading models from assets.hjson... {:?}", assets.models);
        let file_name = &assets.models.get("Floor").unwrap().file;
        let model_path = Path::new("assets/models").join(file_name);
        let mut model = Model::load_model(model_path.as_path());
        model
            .get_material(1)
            .unwrap()
            .set_texture(raylib_aisle9::material::MaterialMapIndex::Albedo, texture);
        models.insert(String::from("floor"), model);
        ASSET_MANAGER
            .set(AssetManager { models, textures })
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

use anyhow::Result;
use raylib_aisle9::camera3d::CameraProjection;
use raylib_aisle9::model::WorldModel;
use raylib_aisle9::vector::Vector3;
use raylib_aisle9::window::Window;
use raylib_aisle9::*;
use raylib_aisle9::{camera3d::Camera3D, colors::*};

use crate::asset_manager::AssetManager;

mod asset_manager;
mod assets_config;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_TITLE: &str = "Aisle9";

pub fn main() -> Result<()> {
    let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
    // Try to fix paths when running directly from the workspace root or the debug/release target directory
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
        && exe_dir.join("assets").exists()
    {
        let _ = std::env::set_current_dir(exe_dir);
    }

    AssetManager::init(); // Initialize the asset manager
    let floor = AssetManager::get().models.get("floor").unwrap();
    let camera = Camera3D::new(
        Vector3::new(50.0, 50.0, 50.0),
        Vector3::origin(),
        Vector3::new(0.0, 1.0, 0.0),
        5.0,
        CameraProjection::CameraOrthographic,
    );

    while !(window.should_close()) {
        draw(|| {
            // Render text and a background
            clear_background(SKYBLUE);
            mode_3d(&camera, || {
                for x in -5..6 {
                    for z in -5..6 {
                        WorldModel::new(floor)
                            .with_position(Vector3 {
                                x: x as f32 * 2.0,
                                y: 0.0,
                                z: z as f32 * 2.0,
                            })
                            .draw_model();
                    }
                }
            });
            draw_fps(10, 10);
        });
    }
    Ok(())
}

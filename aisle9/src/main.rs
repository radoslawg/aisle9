use anyhow::{Context, Result};
use raylib_aisle9::colors::*;
use raylib_aisle9::window::Window;
use raylib_aisle9::*;

use crate::asset_manager::{ASSET_MANAGER, AssetManager};

mod asset_manager;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_TITLE: &str = "Aisle9";

pub fn main() -> Result<()> {
    AssetManager::init(); // Initialize the asset manager
    // Try to fix paths when running directly from the workspace root or the debug/release target directory
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
        && exe_dir.join("resources").exists()
    {
        let _ = std::env::set_current_dir(exe_dir);
    }

    let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
    while !(window.should_close()) {
        draw(|| {
            // Render text and a background
            clear_background(SKYBLUE);
            draw_text("Aisle9! Now in Rust's safe mode!", 210, 200, 20, BLACK);
        });
        draw_fps(10, 10);
    }
    Ok(())
}

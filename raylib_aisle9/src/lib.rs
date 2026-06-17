use raylib_ffi::*;
use std::ffi::CString;

use crate::camera3d::Camera3D;

pub mod camera3d;
pub mod colors;
pub mod consts;
pub mod light;
pub mod material;
pub mod model;
pub mod shader;
pub mod texture2d;
pub mod vector;
pub mod window;

pub fn set_trace_log_level(level: consts::LogLevel) {
    unsafe {
        raylib_ffi::SetTraceLogLevel(level as i32);
    }
}

// **********************************************************
// This is custom logging function implemented by Gemini
// **********************************************************
// use std::ffi::CStr;
// use std::os::raw::{c_char, c_int};
// use log::{trace, debug, info, warn, error};
//
// // 1. Define the platform-specific va_list type
// #[cfg(all(unix, target_arch = "x86_64"))]
// type VaListPtr = *mut raylib_ffi::__va_list_tag;
//
// #[cfg(all(unix, target_arch = "aarch64"))]
// type VaListPtr = raylib_ffi::va_list; // ARM macs/linux often define this differently
//
// #[cfg(target_env = "msvc")]
// type VaListPtr = *mut c_char; // MSVC uses a simple char pointer
//
// #[cfg(target_env = "gnu")]
// type VaListPtr = *mut raylib_ffi::__va_list_tag; // MinGW acts like Linux
//
//
// // 2. Use the alias in your callback signature
// pub unsafe extern "C" fn raylib_to_simplelog_callback(
//     log_level: c_int,
//     text: *const c_char,
//     args: VaListPtr, // <-- Replaced hardcoded type
// ) {
//     if text.is_null() {
//         return;
//     }
//
//     let mut buffer: [u8; 1024] = [0; 1024];
//
//     // vsnprintf works exactly the same on both Windows and Linux in this context
//     libc::vsnprintf(
//         buffer.as_mut_ptr() as *mut c_char,
//         buffer.len(),
//         text,
//         args as *mut _,
//     );
//
//     let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
//     let message = c_str.to_string_lossy();
//
//     match log_level {
//         1 => trace!("{}", message),
//         2 => debug!("{}", message),
//         3 => info!("{}", message),
//         4 => warn!("{}", message),
//         5 | 6 => error!("{}", message),
//         _ => trace!("{}", message),
//     }
// }
pub fn set_trace_log_callback(callback: TraceLogCallback) {
    unsafe {
        raylib_ffi::SetTraceLogCallback(callback);
    }
}

pub fn load_texture(path: &str) -> Texture2D {
    unsafe { raylib_ffi::LoadTexture(CString::new(path).unwrap().as_ptr()) }
}

pub fn clear_background(color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::ClearBackground(color);
    }
}

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::DrawText(
            CString::new(text).unwrap().as_ptr(),
            pos_x,
            pos_y,
            font_size,
            color,
        );
    }
}

pub fn draw<F: FnOnce()>(f: F) {
    unsafe {
        raylib_ffi::BeginDrawing();
        f();
        raylib_ffi::EndDrawing();
    }
}

pub fn mode_3d<F: FnOnce()>(camera: &Camera3D, f: F) {
    unsafe {
        raylib_ffi::BeginMode3D(camera.camera);
        f();
        raylib_ffi::EndMode3D();
    }
}

pub fn set_target_fps(fps: u32) {
    unsafe {
        SetTargetFPS(fps as i32);
    }
}

pub fn get_time() -> f64 {
    unsafe { raylib_ffi::GetTime() }
}

pub fn draw_fps(x: u32, y: u32) {
    unsafe {
        DrawFPS(x as i32, y as i32);
    }
}

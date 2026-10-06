pub mod display;
pub mod overlay;

pub fn register<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        display::render_image,
        display::render_gif,
        overlay::set_overlay_config
    ])
}

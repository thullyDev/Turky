pub mod device;
pub mod display;
pub mod overlay;

pub fn register<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        device::get_display_info,
        display::render_image,
        display::render_gif,
        display::note_presented,
        overlay::set_overlay_config
    ])
}

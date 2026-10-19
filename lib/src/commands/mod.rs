pub mod creation;
pub mod device;
pub mod display;
pub mod font;
pub mod overlay;

pub fn register<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        device::get_display_info,
        display::render_image,
        display::render_gif,
        display::note_presented,
        overlay::set_overlay_config,
        font::download_font,
        font::list_downloaded_fonts,
        creation::list_creations,
        creation::get_creation,
        creation::get_creation_media,
        creation::save_creation,
        creation::rename_creation,
        creation::duplicate_creation,
        creation::delete_creation,
    ])
}

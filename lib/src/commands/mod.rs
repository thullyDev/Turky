pub mod display;

pub fn register<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
) -> tauri::Builder<R> {
    builder.invoke_handler(
        tauri::generate_handler![
            display::render_text,
        ]
    )
}
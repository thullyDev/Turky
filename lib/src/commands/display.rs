use crate::schemas::display_schemas::RenderImageTextResponse;
use crate::state::AppState;

#[tauri::command]
pub fn render_text(
    message: String,
    state: tauri::State<'_, AppState>,
) -> RenderImageTextResponse {
    let mut display = state
        .display
        .lock()
        .expect("Failed to lock DisplayService");

    display.render_text_image(message)
}

#[test]
#[ignore]
fn render_text_returns_message() {
    // TODO: add mock when DisplayService has a device dependency
}

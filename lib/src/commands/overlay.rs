use crate::schemas::overlay_schemas::OverlayConfig;
use crate::state::AppState;

#[tauri::command]
pub fn set_overlay_config(
    config: OverlayConfig,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut display = state
        .display
        .lock()
        .map_err(|e| format!("Failed to lock DisplayService: {e}"))?;

    display.set_overlay_config(config);

    Ok(())
}

#[tauri::command]
pub fn register_font(
    name: String,
    bytes: Vec<u8>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut display = state
        .display
        .lock()
        .map_err(|e| format!("Failed to lock DisplayService: {e}"))?;

    display.register_font(name, bytes)
}

#[tauri::command]
pub fn list_fonts(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let display = state
        .display
        .lock()
        .map_err(|e| format!("Failed to lock DisplayService: {e}"))?;

    Ok(display.font_names())
}

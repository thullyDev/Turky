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
        .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

    display.set_overlay_config(config);

    Ok(())
}

use crate::schemas::display_schemas::DisplayInfo;
use crate::state::AppState;

#[tauri::command]
pub fn get_display_info(state: tauri::State<'_, AppState>) -> Result<Option<DisplayInfo>, String> {
    let mut display = state
        .display
        .lock()
        .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

    let Some(device) = display.device_serv.registry().list().next() else {
        return Ok(None);
    };

    let info = &device.info;

    Ok(Some(DisplayInfo {
        width: info.width,
        height: info.height,
        name: info.name.clone(),
    }))
}

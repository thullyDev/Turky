use tauri::State;

use crate::schemas::font_schemas::DownloadedFontInfo;
use crate::state::AppState;

#[tauri::command]
pub async fn download_font(
    state: State<'_, AppState>,
    font_id: String,
    url: String,
    family: String,
) -> Result<DownloadedFontInfo, String> {
    state.fonts.download(&font_id, &url, &family).await
}

#[tauri::command]
pub fn list_downloaded_fonts(
    state: State<'_, AppState>,
) -> Result<Vec<DownloadedFontInfo>, String> {
    state.fonts.list_downloaded()
}

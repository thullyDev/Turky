use tauri::State;

use crate::schemas::creation_schemas::{
    CreationDocument, CreationKind, CreationSummary, RenameCreationRequest, SaveCreationRequest,
};
use crate::state::AppState;

#[tauri::command]
pub fn list_creations(
    kind: Option<CreationKind>,
    state: State<'_, AppState>,
) -> Result<Vec<CreationSummary>, String> {
    state.creations.list(kind)
}

#[tauri::command]
pub fn get_creation(id: String, state: State<'_, AppState>) -> Result<CreationDocument, String> {
    state.creations.get(&id)
}

#[tauri::command]
pub fn get_creation_media(id: String, state: State<'_, AppState>) -> Result<Option<Vec<u8>>, String> {
    state.creations.get_media(&id)
}

#[tauri::command]
pub fn save_creation(
    request: SaveCreationRequest,
    media: Option<Vec<u8>>,
    state: State<'_, AppState>,
) -> Result<CreationSummary, String> {
    state.creations.save(request, media)
}

#[tauri::command]
pub fn rename_creation(
    request: RenameCreationRequest,
    state: State<'_, AppState>,
) -> Result<CreationSummary, String> {
    state.creations.rename(request)
}

#[tauri::command]
pub fn duplicate_creation(
    id: String,
    state: State<'_, AppState>,
) -> Result<CreationSummary, String> {
    state.creations.duplicate(&id)
}

#[tauri::command]
pub fn delete_creation(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.creations.delete(&id)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreationKind {
    Draft,
    Saved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationSummary {
    pub id: String,
    pub title: String,
    pub kind: CreationKind,
    pub display_width: u32,
    pub display_height: u32,
    pub updated_at: String,
    pub created_at: String,
    pub has_media: bool,
    pub media_type: Option<String>,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationDocument {
    pub summary: CreationSummary,
    pub items: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCreationRequest {
    pub id: Option<String>,
    pub title: String,
    pub kind: CreationKind,
    pub display_width: u32,
    pub display_height: u32,
    pub items: serde_json::Value,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCreationRequest {
    pub id: String,
    pub title: String,
}

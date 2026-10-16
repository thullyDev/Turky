use serde::Serialize;

#[derive(Serialize)]
pub struct RenderImageTextResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub name: String,
}

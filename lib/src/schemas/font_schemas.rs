use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadedFontInfo {
    pub id: String,
    pub family: String,
    pub path: String,
}

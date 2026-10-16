use reqwest::Client;

use crate::schemas::font_schemas::DownloadedFontInfo;

use super::font_store::FontStore;

pub struct FontService {
    store: FontStore,
    client: Client,
}

impl FontService {
    pub fn new(store: FontStore, client: Client) -> Self {
        Self { store, client }
    }

    pub async fn download(
        &self,
        font_id: &str,
        url: &str,
        family: &str,
    ) -> Result<DownloadedFontInfo, String> {
        let id = sanitize_id(font_id)?;

        if !self.store.font_exists(&id) {
            let response = self
                .client
                .get(url.trim())
                .send()
                .await
                .map_err(|error| format!("Download failed: {error}"))?;

            if !response.status().is_success() {
                return Err(format!("Download failed with status {}", response.status()));
            }

            let bytes = response
                .bytes()
                .await
                .map_err(|error| format!("Failed to read font bytes: {error}"))?;

            self.store.save_font(&id, &bytes, family)?;
        }

        let family_name = self
            .store
            .read_family_name(&id)
            .unwrap_or_else(|| family.trim().to_string());
        let path = self.store.font_path(&id).to_string_lossy().into_owned();

        Ok(DownloadedFontInfo {
            id,
            family: family_name,
            path,
        })
    }

    pub fn list_downloaded(&self) -> Result<Vec<DownloadedFontInfo>, String> {
        self.store.list()
    }
}

fn sanitize_id(font_id: &str) -> Result<String, String> {
    let trimmed = font_id.trim();

    if trimmed.is_empty() || trimmed.len() > 64 {
        return Err("Invalid font id.".to_string());
    }

    if !trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
    {
        return Err("Font id contains invalid characters.".to_string());
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_font_ids() {
        assert!(sanitize_id("").is_err());
        assert!(sanitize_id("../evil").is_err());
        assert_eq!(sanitize_id("orbitron").expect("valid id"), "orbitron");
    }
}

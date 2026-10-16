use std::fs;
use std::path::PathBuf;

use crate::schemas::font_schemas::DownloadedFontInfo;

pub struct FontStore {
    root: PathBuf,
}

impl FontStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn init(&self) -> Result<(), String> {
        fs::create_dir_all(&self.root)
            .map_err(|error| format!("Failed to create fonts dir: {error}"))
    }

    pub fn font_path(&self, id: &str) -> PathBuf {
        self.root.join(format!("{id}.ttf"))
    }

    pub fn save_font(&self, id: &str, bytes: &[u8], family: &str) -> Result<(), String> {
        if bytes.len() < 1024 {
            return Err("Downloaded file looks too small to be a font.".to_string());
        }

        fs::write(self.font_path(id), bytes)
            .map_err(|error| format!("Failed to save font: {error}"))?;

        let meta = serde_json::json!({ "family": family.trim() });
        fs::write(
            self.root.join(format!("{id}.json")),
            serde_json::to_vec(&meta)
                .map_err(|error| format!("Failed to write font meta: {error}"))?,
        )
        .map_err(|error| format!("Failed to save font meta: {error}"))?;

        Ok(())
    }

    pub fn font_exists(&self, id: &str) -> bool {
        self.font_path(id).exists()
    }

    pub fn list(&self) -> Result<Vec<DownloadedFontInfo>, String> {
        let mut fonts = Vec::new();

        let entries = fs::read_dir(&self.root)
            .map_err(|error| format!("Failed to read fonts dir: {error}"))?;

        for entry in entries {
            let entry = entry.map_err(|error| format!("Failed to read font entry: {error}"))?;
            let path = entry.path();

            if path.extension().and_then(|value| value.to_str()) != Some("ttf") {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
                continue;
            };

            let family = self
                .read_family_name(stem)
                .unwrap_or_else(|| stem.replace('-', " "));

            fonts.push(DownloadedFontInfo {
                id: stem.to_string(),
                family,
                path: path.to_string_lossy().into_owned(),
            });
        }

        fonts.sort_by(|left, right| left.id.cmp(&right.id));

        Ok(fonts)
    }

    pub fn read_family_name(&self, id: &str) -> Option<String> {
        let meta_path = self.root.join(format!("{id}.json"));
        let bytes = fs::read(meta_path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("family")
            .and_then(|entry| entry.as_str())
            .map(str::to_string)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_lists_font_metadata() {
        let dir = std::env::temp_dir().join(format!("turky-font-store-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);

        let store = FontStore::new(dir.clone());
        store.init().expect("init should succeed");

        let bytes = vec![0_u8; 2048];
        store
            .save_font("orbitron", &bytes, "Orbitron")
            .expect("save should succeed");

        let fonts = store.list().expect("list should succeed");

        assert_eq!(fonts.len(), 1);
        assert_eq!(fonts[0].id, "orbitron");
        assert_eq!(fonts[0].family, "Orbitron");

        let _ = fs::remove_dir_all(&dir);
    }
}

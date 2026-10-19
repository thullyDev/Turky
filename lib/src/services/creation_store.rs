use std::fs;
use std::path::{Path, PathBuf};

use crate::schemas::creation_schemas::{CreationDocument, CreationKind, CreationSummary};

const MANIFEST: &str = "manifest.json";
const EDITOR: &str = "editor.json";
const MEDIA: &str = "media.bin";
const THUMBNAIL: &str = "thumbnail.jpg";

pub struct CreationStore {
    root: PathBuf,
}

impl CreationStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn init(&self) -> Result<(), String> {
        fs::create_dir_all(&self.root)
            .map_err(|error| format!("Failed to create creations dir: {error}"))
    }

    pub fn list(&self, kind: Option<CreationKind>) -> Result<Vec<CreationSummary>, String> {
        let mut summaries = Vec::new();

        if !self.root.exists() {
            return Ok(summaries);
        }

        for entry in fs::read_dir(&self.root)
            .map_err(|error| format!("Failed to read creations dir: {error}"))?
        {
            let entry = entry.map_err(|error| format!("Failed to read creation entry: {error}"))?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let Some(summary) = self.read_manifest(&path)? else {
                continue;
            };

            if kind.is_some_and(|filter| summary.kind != filter) {
                continue;
            }

            summaries.push(summary);
        }

        summaries.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));

        Ok(summaries)
    }

    pub fn read(&self, id: &str) -> Result<CreationDocument, String> {
        let dir = self.creation_dir(id)?;

        let summary = self
            .read_manifest(&dir)?
            .ok_or_else(|| format!("Creation {id} not found"))?;

        let editor_path = dir.join(EDITOR);
        let items = if editor_path.exists() {
            let bytes = fs::read(&editor_path)
                .map_err(|error| format!("Failed to read editor state: {error}"))?;
            serde_json::from_slice(&bytes)
                .map_err(|error| format!("Failed to parse editor state: {error}"))?
        } else {
            serde_json::json!([])
        };

        Ok(CreationDocument { summary, items })
    }

    pub fn write(
        &self,
        summary: &CreationSummary,
        items: &serde_json::Value,
        media: Option<&[u8]>,
        thumbnail: Option<&[u8]>,
    ) -> Result<(), String> {
        let dir = self.creation_dir(&summary.id)?;
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Failed to create creation dir: {error}"))?;

        let manifest_bytes = serde_json::to_vec_pretty(summary)
            .map_err(|error| format!("Failed to encode manifest: {error}"))?;
        fs::write(dir.join(MANIFEST), manifest_bytes)
            .map_err(|error| format!("Failed to write manifest: {error}"))?;

        let editor_bytes = serde_json::to_vec_pretty(items)
            .map_err(|error| format!("Failed to encode editor state: {error}"))?;
        fs::write(dir.join(EDITOR), editor_bytes)
            .map_err(|error| format!("Failed to write editor state: {error}"))?;

        let media_path = dir.join(MEDIA);
        if let Some(bytes) = media {
            fs::write(&media_path, bytes)
                .map_err(|error| format!("Failed to write media: {error}"))?;
        } else if !summary.has_media && media_path.exists() {
            fs::remove_file(&media_path)
                .map_err(|error| format!("Failed to remove media: {error}"))?;
        }

        let thumb_path = dir.join(THUMBNAIL);
        if let Some(bytes) = thumbnail {
            fs::write(&thumb_path, bytes)
                .map_err(|error| format!("Failed to write thumbnail: {error}"))?;
        } else if thumbnail.is_none() && !summary.has_media && thumb_path.exists() {
            let _ = fs::remove_file(&thumb_path);
        }

        Ok(())
    }

    pub fn read_media(&self, id: &str) -> Result<Option<Vec<u8>>, String> {
        let path = self.creation_dir(id)?.join(MEDIA);

        if !path.exists() {
            return Ok(None);
        }

        fs::read(&path)
            .map(Some)
            .map_err(|error| format!("Failed to read media: {error}"))
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let dir = self.creation_dir(id)?;

        if !dir.exists() {
            return Err(format!("Creation {id} not found"));
        }

        fs::remove_dir_all(&dir).map_err(|error| format!("Failed to delete creation: {error}"))
    }

    pub fn exists(&self, id: &str) -> bool {
        self.creation_dir(id)
            .ok()
            .is_some_and(|dir| dir.join(MANIFEST).exists())
    }

    fn creation_dir(&self, id: &str) -> Result<PathBuf, String> {
        if id.is_empty() || id.contains(['/', '\\']) || id.contains("..") {
            return Err("Invalid creation id".into());
        }

        Ok(self.root.join(id))
    }

    fn read_manifest(&self, dir: &Path) -> Result<Option<CreationSummary>, String> {
        let path = dir.join(MANIFEST);

        if !path.exists() {
            return Ok(None);
        }

        let bytes = fs::read(&path).map_err(|error| format!("Failed to read manifest: {error}"))?;
        let mut summary: CreationSummary = serde_json::from_slice(&bytes)
            .map_err(|error| format!("Failed to parse manifest: {error}"))?;

        let thumb = dir.join(THUMBNAIL);
        summary.thumbnail_path = thumb.exists().then(|| thumb.to_string_lossy().into_owned());

        Ok(Some(summary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::creation_schemas::CreationKind;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_store() -> (CreationStore, PathBuf) {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("turky-creations-{nanos}"));
        let store = CreationStore::new(root.clone());
        store.init().expect("init");
        (store, root)
    }

    fn sample_summary(id: &str, kind: CreationKind) -> CreationSummary {
        CreationSummary {
            id: id.to_string(),
            title: "Test pack".into(),
            kind,
            display_width: 480,
            display_height: 1920,
            updated_at: "2026-01-01T00:00:00Z".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
            has_media: false,
            media_type: None,
            thumbnail_path: None,
        }
    }

    #[test]
    fn saves_and_lists_creations_by_kind() {
        let (store, root) = temp_store();

        store
            .write(
                &sample_summary("a", CreationKind::Draft),
                &serde_json::json!([{"metric":"cpu"}]),
                None,
                None,
            )
            .expect("write draft");
        store
            .write(
                &sample_summary("b", CreationKind::Saved),
                &serde_json::json!([]),
                None,
                None,
            )
            .expect("write saved");

        let drafts = store.list(Some(CreationKind::Draft)).expect("list drafts");
        let saved = store.list(Some(CreationKind::Saved)).expect("list saved");

        assert_eq!(drafts.len(), 1);
        assert_eq!(saved.len(), 1);
        assert_eq!(drafts[0].id, "a");

        let _ = fs::remove_dir_all(root);
    }
}

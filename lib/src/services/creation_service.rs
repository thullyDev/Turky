use std::io::Cursor;

use chrono::Utc;
use image::codecs::jpeg::JpegEncoder;
use image::DynamicImage;

use crate::schemas::creation_schemas::{
    CreationDocument, CreationKind, CreationSummary, RenameCreationRequest, SaveCreationRequest,
};
use crate::services::creation_store::CreationStore;
use crate::services::frame_fit;

pub struct CreationService {
    store: CreationStore,
}

impl CreationService {
    pub fn new(store: CreationStore) -> Self {
        Self { store }
    }

    pub fn init(&self) -> Result<(), String> {
        self.store.init()
    }

    pub fn list(&self, kind: Option<CreationKind>) -> Result<Vec<CreationSummary>, String> {
        self.store.list(kind)
    }

    pub fn get(&self, id: &str) -> Result<CreationDocument, String> {
        self.store.read(id)
    }

    pub fn get_media(&self, id: &str) -> Result<Option<Vec<u8>>, String> {
        self.store.read_media(id)
    }

    pub fn save(
        &self,
        request: SaveCreationRequest,
        media: Option<Vec<u8>>,
    ) -> Result<CreationSummary, String> {
        let now = Utc::now().to_rfc3339();
        let id = request.id.clone().unwrap_or_else(new_creation_id);

        let created_at = if request.id.as_ref().is_some_and(|existing| self.store.exists(existing))
        {
            self.store
                .read(&id)
                .map(|doc| doc.summary.created_at)
                .unwrap_or_else(|_| now.clone())
        } else {
            now.clone()
        };

        let media_bytes = match media {
            Some(bytes) if !bytes.is_empty() => Some(bytes),
            Some(_) => None,
            None if request.id.as_ref().is_some_and(|existing| self.store.exists(existing)) => {
                self.store.read_media(&id)?
            }
            None => None,
        };

        let has_media = media_bytes.is_some();
        let thumbnail = media_bytes
            .as_deref()
            .and_then(|bytes| build_thumbnail(bytes, request.media_type.as_deref()));

        let title = match request.kind {
            CreationKind::Saved => sanitize_saved_title(&request.title)?,
            CreationKind::Draft => sanitize_draft_title(&request.title),
        };

        let mut summary = CreationSummary {
            id: id.clone(),
            title,
            kind: request.kind,
            display_width: request.display_width.max(1),
            display_height: request.display_height.max(1),
            updated_at: now,
            created_at,
            has_media,
            media_type: if has_media {
                request.media_type.clone()
            } else {
                None
            },
            thumbnail_path: None,
        };

        self.store.write(
            &summary,
            &request.items,
            media_bytes.as_deref(),
            thumbnail.as_deref(),
        )?;

        if let Some(document) = self.store.read(&id).ok() {
            summary = document.summary;
        }

        Ok(summary)
    }

    pub fn rename(&self, request: RenameCreationRequest) -> Result<CreationSummary, String> {
        let mut document = self.store.read(&request.id)?;
        document.summary.title = sanitize_saved_title(&request.title)?;
        document.summary.updated_at = Utc::now().to_rfc3339();

        self.store.write(
            &document.summary,
            &document.items,
            self.store.read_media(&request.id)?.as_deref(),
            None,
        )?;

        self.store.read(&request.id).map(|doc| doc.summary)
    }

    pub fn duplicate(&self, id: &str) -> Result<CreationSummary, String> {
        let source = self.store.read(id)?;
        let media = self.store.read_media(id)?;

        let request = SaveCreationRequest {
            id: None,
            title: format!("{} (copy)", source.summary.title),
            kind: source.summary.kind,
            display_width: source.summary.display_width,
            display_height: source.summary.display_height,
            items: source.items,
            media_type: source.summary.media_type.clone(),
        };

        self.save(request, media)
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        self.store.delete(id)
    }
}

fn new_creation_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or(0);
    format!("creation-{nanos:x}")
}

fn sanitize_draft_title(title: &str) -> String {
    title.trim().to_string()
}

fn sanitize_saved_title(title: &str) -> Result<String, String> {
    let trimmed = title.trim();

    if trimmed.is_empty() {
        return Err("Name your creation before saving.".into());
    }

    Ok(trimmed.to_string())
}

fn build_thumbnail(bytes: &[u8], media_type: Option<&str>) -> Option<Vec<u8>> {
    if media_type == Some("image/gif") {
        return None;
    }

    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let fitted = frame_fit::fit_cover(&image, 320, 180);
    let dynamic = DynamicImage::ImageRgba8(fitted);

    let mut output = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(Cursor::new(&mut output), 80);
    encoder
        .encode(
            dynamic.as_bytes(),
            dynamic.width(),
            dynamic.height(),
            dynamic.color().into(),
        )
        .ok()?;

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::creation_schemas::CreationKind;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_service() -> (CreationService, std::path::PathBuf) {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("turky-creation-svc-{nanos}"));
        let store = CreationStore::new(root.clone());
        let service = CreationService::new(store);
        service.init().expect("init");
        (service, root)
    }

    #[test]
    fn duplicate_creates_new_saved_creation() {
        let (service, root) = temp_service();

        let saved = service
            .save(
                SaveCreationRequest {
                    id: None,
                    title: "Original".into(),
                    kind: CreationKind::Saved,
                    display_width: 480,
                    display_height: 1920,
                    items: serde_json::json!([{"metric":"cpu","enabled":true}]),
                    media_type: None,
                },
                None,
            )
            .expect("save");

        let copy = service.duplicate(&saved.id).expect("duplicate");

        assert_ne!(saved.id, copy.id);
        assert!(copy.title.contains("copy"));

        let _ = std::fs::remove_dir_all(root);
    }
}

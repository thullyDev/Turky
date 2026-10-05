use std::collections::HashMap;

use ab_glyph::FontArc;

use crate::schemas::overlay_schemas::DEFAULT_FONT_NAME;

pub struct FontRegistry {
    fonts: HashMap<String, FontArc>,
}

impl FontRegistry {
    pub fn bundled() -> Self {
        let mut registry = Self {
            fonts: HashMap::new(),
        };

        registry
            .register(
                DEFAULT_FONT_NAME.to_string(),
                include_bytes!("../../assets/fonts/Roboto-Regular.ttf").to_vec(),
            )
            .expect("bundled Roboto font should load");

        registry
    }

    pub fn register(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        let font = FontArc::try_from_vec(bytes)
            .map_err(|error| format!("Failed to load font: {error}"))?;

        self.fonts.insert(name, font);

        Ok(())
    }

    pub fn get(&self, name: &str) -> &FontArc {
        let requested = if name.is_empty() { DEFAULT_FONT_NAME } else { name };

        self.fonts
            .get(requested)
            .or_else(|| self.fonts.get(DEFAULT_FONT_NAME))
            .expect("default font is registered")
    }

    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.fonts.keys().cloned().collect();

        names.sort();

        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_registry_includes_roboto() {
        let registry = FontRegistry::bundled();

        assert_eq!(registry.names(), vec!["Roboto".to_string()]);
    }

    #[test]
    fn missing_font_falls_back_to_roboto() {
        let registry = FontRegistry::bundled();

        let fallback = registry.get("does-not-exist");
        let roboto = registry.get("Roboto");

        assert!(std::ptr::eq(fallback, roboto));
    }

    #[test]
    fn rejects_invalid_font_bytes() {
        let mut registry = FontRegistry::bundled();

        let error = registry
            .register("Broken".to_string(), b"not a font".to_vec())
            .expect_err("invalid font should fail");

        assert!(error.contains("Failed to load font"));
        assert!(!registry.names().iter().any(|name| name == "Broken"));
    }
}

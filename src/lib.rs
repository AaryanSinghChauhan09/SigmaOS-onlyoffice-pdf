//! SigmaOS-onlyoffice-pdf: ONLYOFFICE PDF viewer/editor model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for ONLYOFFICE PDF viewer/editor model.
pub struct OoPdfDoc {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

/// Secondary domain entity for ONLYOFFICE PDF viewer/editor model.
pub struct OoAnnotation {
    pub id: String,
    pub annotation_type: String,
    pub description: String,
}

/// Errors for the ONLYOFFICE PDF viewer/editor model domain.
#[derive(Debug, PartialEq)]
pub enum OnlyofficePdfError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for OnlyofficePdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for OnlyofficePdfError {}

/// Registry and manager for ONLYOFFICE PDF viewer/editor model.
pub struct OnlyofficePdfManager {
    pub name: String,
    items: BTreeMap<String, OoPdfDoc>,
    configs: BTreeMap<String, OoAnnotation>,
}

impl OnlyofficePdfManager {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: OoPdfDoc) -> Result<(), OnlyofficePdfError> {
        if item.name.is_empty() {
            return Err(OnlyofficePdfError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(OnlyofficePdfError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Option<&OoPdfDoc> {
        self.items.get(id)
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
    pub fn add_config(&mut self, cfg: OoAnnotation) {
        self.configs.insert(cfg.id.clone(), cfg);
    }
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), OnlyofficePdfError> {
        self.items
            .get_mut(id)
            .map(|i| i.enabled = enabled)
            .ok_or_else(|| OnlyofficePdfError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str, name: &str) -> OoPdfDoc {
        OoPdfDoc {
            id: id.to_string(),
            name: name.to_string(),
            version: "default".to_string(),
            enabled: true,
        }
    }

    fn make_config(id: &str) -> OoAnnotation {
        OoAnnotation {
            id: id.to_string(),
            annotation_type: "val".to_string(),
            description: format!("{} desc", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = OnlyofficePdfManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        assert!(mgr.add_item(make_item("i1", "First")).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        assert_eq!(
            mgr.add_item(make_item("i1", "")),
            Err(OnlyofficePdfError::EmptyName)
        );
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        mgr.add_item(make_item("i1", "First")).unwrap();
        assert_eq!(
            mgr.add_item(make_item("i1", "Dup")),
            Err(OnlyofficePdfError::Duplicate("i1".to_string()))
        );
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        mgr.add_config(make_config("c1"));
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        mgr.add_item(make_item("i1", "First")).unwrap();
        assert!(mgr.get_item("i1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = OnlyofficePdfManager::new("test");
        mgr.add_item(make_item("i1", "First")).unwrap();
        assert!(mgr.set_enabled("i1", false).is_ok());
        assert!(!mgr.get_item("i1").unwrap().enabled);
        assert_eq!(
            mgr.set_enabled("none", true),
            Err(OnlyofficePdfError::NotFound("none".to_string()))
        );
    }
}

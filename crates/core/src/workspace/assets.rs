use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetCatalog {
    assets: HashMap<AssetRef, AssetMetadata>,
}

impl AssetCatalog {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: impl AsRef<Path>) -> AssetRef {
        let id = AssetRef::new();
        self.assets.insert(
            id,
            AssetMetadata {
                id,
                path: path.as_ref().to_path_buf(),
            },
        );
        id
    }

    pub fn get(&self, id: AssetRef) -> Option<&AssetMetadata> {
        self.assets.get(&id)
    }
}

impl Default for AssetCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetRef(uuid::Uuid);

impl AssetRef {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: AssetRef,
    pub path: PathBuf,
}

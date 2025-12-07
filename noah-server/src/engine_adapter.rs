use nanodb_core::{NanoDb, DbResult};
use serde::de::value;
use std::sync::Arc;

// Wrapper para adaptar NanoDB real a la API síncrona de NoahDB
pub struct NanoEngine {
    db: Arc<NanoDb>,
}

impl NanoEngine {
    pub fn new() -> Self {
        Self {
            db: Arc::new(NanoDb::new()),
        }
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        let db = Arc::clone(&self.db);
        let key = key.to_string();
        let handle = tokio::spawn(async move {
            db.set(key, value).await;
        });

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(handle).ok();
        });
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let db = Arc::clone(&self.db);
        let key = key.to_string();
        let handle = tokio::spawn(async move {
            match db.get(&key).await {
                DbResult::Ok(data) => Some(data),
                _ => None,
            }
        });
        
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(handle).ok().flatten()
        })
    }

    pub fn delete(&self, key: &str) -> bool {
        let db = Arc::clone(&self.db);
        let key = key.to_string();
        let handle = tokio::spawn(async move {
            matches!(db.delete(&key).await, DbResult::Ok(_))
        });
        
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(handle).unwrap_or(false)
        })
    }

    pub fn list_keys(&self, prefix: &str) -> Vec<String> {
        let db = Arc::clone(&self.db);
        let prefix = prefix.to_string();
        let handle = tokio::spawn(async move {
            match db.keys().await {
                DbResult::Ok(keys) => {
                    keys.into_iter()
                        .filter(|k| k.starts_with(&prefix))
                        .collect()
                },
                _ => Vec::new(),
            }
        });
        
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(handle).unwrap_or_default()
        })
    }

    pub fn get_all_data(&self) -> Vec<(String, Vec<u8>)> {
        let keys = self.list_keys("");
        let mut data = Vec::new();

        for key in keys {
            if let Some(value) = self.get(&key) {
                data.push((key, value));
            }
        }
        data
    }
}

impl std::fmt::Debug for NanoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NanoEngine")
            .field("engine", &"NanoDB")
            .finish()
    }
}

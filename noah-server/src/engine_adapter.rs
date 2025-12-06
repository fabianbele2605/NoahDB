use nanodb_core::{NanoDb, DbResult};

// Wrapper para adaptar NanoDB real a la API síncrona de NoahDB
pub struct NanoEngine {
    db: NanoDb,
    runtime: tokio::runtime::Runtime,
}

impl NanoEngine {
    pub fn new() -> Self {
        Self {
            db: NanoDb::new(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.runtime.block_on(async {
            self.db.set(key.to_string(), value).await
        });
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.runtime.block_on(async {
            match self.db.get(key).await {
                DbResult::Ok(data) => Some(data),
                _ => None,
            }
        })
    }

    pub fn delete(&self, key: &str) -> bool {
        self.runtime.block_on(async {
            matches!(self.db.delete(key).await, DbResult::Ok(_))
        })
    }

    pub fn list_keys(&self, prefix: &str) -> Vec<String> {
        self.runtime.block_on(async {
            match self.db.keys().await {
                DbResult::Ok(keys) => {
                    keys.into_iter()
                        .filter(|k| k.starts_with(prefix))
                        .collect()
                },
                _ => Vec::new(),
            }
        })
    }
}

impl std::fmt::Debug for NanoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NanoEngine")
            .field("engine", &"NanoDB")
            .finish()
    }
}
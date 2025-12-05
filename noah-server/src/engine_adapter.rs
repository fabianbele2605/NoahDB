// Importar NanoDB desde el repo existente
// Nota: Esto asume que NanoDB está disponible como dependencia
// Si no, necesitaremos ajustar el path o la dependencia

// Wrapper para adaptar NanoDB a la API que espera NoahDB
pub struct NanoEngine {
    // Por ahora usamos DashMap directamente hasta que tengamos NanoDB como dependencia
    storage: dashmap::DashMap<String, Vec<u8>>,
}

impl NanoEngine {
    pub fn new() -> Self {
        Self {
            storage: dashmap::DashMap::new(),
        }
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.storage.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.storage.get(key).map(|v| v.clone())
    }

    pub fn delete(&self, key: &str) -> bool {
        self.storage.remove(key).is_some()
    }

    pub fn list_keys(&self, prefix: &str) -> Vec<String> {
        self.storage
            .iter()
            .filter(|entry| entry.key().starts_with(prefix))
            .map(|entry| entry.key().clone())
            .collect()
    }
}

impl std::fmt::Debug for NanoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NanoEngine")
            .field("keys_count", &self.storage.len())
            .finish()
    }
}
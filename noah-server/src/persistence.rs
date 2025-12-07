use std::fs::File;
use std::io::{Write, BufReader};
use serde::{Serialize, Deserialize};

// Estructura para guardar un snapshot
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: u64,
    pub data: Vec<(String, Vec<u8>)>,
}

// Estructura para el log AOF
#[derive(Serialize, Deserialize)]
pub enum Operation {
    Set { key: String, value: Vec<u8> },
    Delete { key: String },
}

// Estructura para el manejador de persistencia
pub struct PersistenceManager {
    snapshot_path: String,
    aof_path: String,
}

impl PersistenceManager {
    pub fn new(data_dir: &str) -> Self {
        // Crear directorio si no existe
        std::fs::create_dir_all(data_dir).ok();

        Self {
            snapshot_path: format!("{}/snapshot.rdb", data_dir),
            aof_path: format!("{}/appendonly.aof", data_dir),
        }
    }

    //  Guardar snapshot de todos los datos
    pub fn save_snapshot(&self, data: Vec<(String, Vec<u8>)>) -> std::io::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let snapshot = Snapshot { timestamp, data };

        // Guardar en formato binario (mas rapido y compacto)
        let encoded = bincode::serialize(&snapshot)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = File::create(&self.snapshot_path)?;
        file.write_all(&encoded)?;

        tracing::info!("Snapshot saved: {} keys", snapshot.data.len());
        Ok(())
    }

    // Cargar snapshot desde disco
    pub fn load_snapshot(&self) -> std::io::Result<Vec<(String, Vec<u8>)>> {
        if !std::path::Path::new(&self.snapshot_path).exists() {
            tracing::info!("No snapshot found, starting fresh");
            return Ok(Vec::new());
        }

        let file = File::open(&self.snapshot_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut buffer)?;

        let snapshot: Snapshot = bincode::deserialize(&buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        tracing::info!(" Snapshot loaded: {} keys", snapshot.data.len());
        Ok(snapshot.data)

    }
}
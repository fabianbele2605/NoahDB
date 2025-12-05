Markdown

# 🏗️ NanoDB Ecosystem: Roadmap de Arquitectura Modular

> **Documento de Diseño Técnico**
> **Objetivo:** Evolucionar el proyecto NanoDB de un servidor monolítico a un ecosistema de herramientas de alto rendimiento desacopladas.
> **Enfoque:** Separation of Concerns (SoC), Reusabilidad de Código, High-Performance Testing.

---

## 🗺️ Visión de Arquitectura

El proyecto se reestructurará utilizando un patrón de **Cargo Workspace** (o multi-repo vinculado) para separar la definición de protocolos de la implementación del servidor y las herramientas de cliente.

```mermaid
graph TD
    A[📦 nanodb-protocol] -->|Define Tipos y Protos| B(🏛️ nanodb-server)
    A -->|Define Tipos y Protos| C(🔨 nano-bench)
    
    B -->|Implementa| D[Core: DashMap + Tokio]
    C -->|Genera Carga| E[Stress Testing: Async Clients]
Componentes del Ecosistema
nanodb-protocol (Shared Lib): "La Verdad Única". Contiene definiciones gRPC (.proto), serialización binaria TCP y enums de error.

nanodb-server (Backend): El motor de base de datos actual. Importa la librería para procesar peticiones.

nano-bench (CLI Tool): Nueva herramienta de benchmarking. Importa la librería para generar tráfico masivo y medir latencia/throughput.

🚀 Fase 1: Extracción del Núcleo (nanodb-protocol)
Objetivo: Crear una librería pura que no dependa de lógica de negocio, solo de definiciones de datos.

1.1 Inicialización
Crear la librería en la raíz del workspace:

Bash

cargo new --lib nanodb-protocol
1.2 Dependencias (nanodb-protocol/Cargo.toml)
Solo necesitamos librerías para definir y serializar datos.

Ini, TOML

[package]
name = "nanodb-protocol"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
prost = "0.12"           # Runtime de Protocol Buffers
bytes = "1.5"            # Manejo eficiente de buffers TCP
thiserror = "1.0"        # Manejo de errores idiomático

[build-dependencies]
tonic-build = "0.10"     # Para compilar los .proto en build.rs
1.3 Migración de Código
Mover .proto: Trasladar la carpeta proto/ desde el servidor a nanodb-protocol/proto/.

Configurar build.rs: Crear un script de compilación en la librería para generar el código Rust de los protos.

Definir Structs TCP: Si tienes structs como MessageHeader o Command, moverlos a src/lib.rs o src/tcp.rs dentro de esta librería.

🔄 Fase 2: Refactorización del Servidor (nanodb-server)
Objetivo: Limpiar el código del servidor para que delegue la definición de tipos a la librería.

2.1 Inyección de Dependencia (Cargo.toml)
Ini, TOML

[dependencies]
# Referencia local para desarrollo rápido
nanodb-protocol = { path = "../nanodb-protocol" }

tokio = { version = "1", features = ["full"] }
tonic = "0.10"
# ... resto de dependencias
2.2 Limpieza (src/main.rs & módulos)
Eliminar: Borrar la generación de protos local (tonic::include_proto!) del main.rs.

Importar: Cambiar las referencias para usar la librería externa.

Rust

// ANTES:
// use crate::proto::nanodb_service_server::NanodbService;

// AHORA:
use nanodb_protocol::grpc::nanodb_service_server::NanodbService;
🔨 Fase 3: Construcción de la Herramienta (nano-bench)
Objetivo: Crear un CLI en Rust capaz de saturar el servidor usando concurrencia de Tokio.

3.1 Inicialización
Bash

cargo new --bin nano-bench
3.2 Stack Tecnológico (nano-bench/Cargo.toml)
Ini, TOML

[dependencies]
nanodb-protocol = { path = "../nanodb-protocol" } # Reutilizamos la lógica!
tokio = { version = "1", features = ["full"] }
clap = { version = "4.4", features = ["derive"] } # Parsing de argumentos CLI
hdrhistogram = "7.5" # Para medir latencia percentil (P99) profesionalmente
rand = "0.8"         # Generación de datos aleatorios
colored = "2.0"      # Output bonito en terminal
3.3 Estructura Base (src/main.rs)
Rust

use clap::Parser;
use std::sync::Arc;
use tokio::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "NanoBench")]
#[command(about = "Herramienta de Stress Testing para NanoDB", long_about = None)]
struct Args {
    /// Dirección del target (ej. 127.0.0.1:8080)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    target: String,

    /// Protocolo a testear (tcp, grpc, http)
    #[arg(short, long, default_value = "tcp")]
    protocol: String,

    /// Nivel de concurrencia (Virtual Users)
    #[arg(short, long, default_value_t = 500)]
    concurrency: usize,
    
    /// Número de peticiones por hilo
    #[arg(short, long, default_value_t = 1000)]
    requests: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("🔥 Iniciando ataque a {} vía {} con {} hilos...", 
             args.target, args.protocol, args.concurrency);

    let start = Instant::now();
    
    // Aquí implementaremos el loop de ataque usando tokio::spawn
    // reutilizando los tipos de nanodb-protocol
    
    let duration = start.elapsed();
    println!("✅ Benchmark finalizado en {:.2?}", duration);
}
🏆 Valor Profesional (Por qué hacemos esto)
Al completar esta migración, el portafolio demostrará:

Ingeniería de Sistemas: Capacidad para manejar Workspaces de Rust y dependencias locales.

Arquitectura Limpia: Principio DRY (Don't Repeat Yourself) aplicado a definiciones de protocolos.

Tooling: Creación de herramientas de desarrollo ("Developer Experience") propias.

Performance: nano-bench servirá como prueba irrefutable de la velocidad de NanoDB.

✅ Checklist de Implementación
[ ] Crear crate nanodb-protocol.

[ ] Mover definiciones .proto y compilar librería.

[ ] Refactorizar nanodb-server para usar la librería.

[ ] Verificar que el servidor compila y pasan los tests.

[ ] Crear crate nano-bench.

[ ] Implementar cliente básico TCP en el benchmark.

[ ] Publicar resultados de rendimiento en el README.
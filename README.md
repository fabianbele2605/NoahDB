# 🚀 NoahDB Ecosystem

> **Base de datos en memoria ultra-rápida con herramientas de benchmarking profesional**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![gRPC](https://img.shields.io/badge/gRPC-Protocol%20Buffers-blue.svg)](https://grpc.io)
[![Performance](https://img.shields.io/badge/Performance-8K%20req%2Fs-green.svg)](#-rendimiento)

## 📋 Descripción

NoahDB es un ecosistema modular de base de datos en memoria construido en Rust, diseñado para máximo rendimiento y escalabilidad. Utiliza gRPC para comunicación de alta velocidad y proporciona herramientas profesionales de benchmarking.

## 🏗️ Arquitectura

```
NoahDB Ecosystem
├── noah-protocol/     # 📦 Librería compartida (gRPC + Protocol Buffers)
├── noah-server/       # 🏛️ Servidor de base de datos
└── noah-bench/        # 🔨 Herramienta de benchmarking
```

### Componentes

- **`noah-protocol`**: Librería compartida que define las APIs gRPC y tipos de datos
- **`noah-server`**: Servidor de base de datos con storage thread-safe usando DashMap
- **`noah-bench`**: Cliente de benchmarking con métricas profesionales (P50/P95/P99)

## ⚡ Características

- 🚀 **Ultra-rápido**: Hasta 8,000+ req/s en cargas intensivas
- 🔒 **Thread-safe**: Concurrencia masiva sin locks explícitos
- 📊 **Métricas profesionales**: Latencias P50, P95, P99
- 🛠️ **Modular**: Arquitectura de Cargo Workspace
- 🌐 **gRPC**: Comunicación de alta performance
- ⚙️ **Configurable**: CLI flexible para diferentes cargas de trabajo

## 🚀 Inicio Rápido

### Prerrequisitos

- Rust 1.70+
- Cargo

### Instalación

```bash
git clone <tu-repo>
cd nanoEcosytem
cargo build --release
```

### Uso Básico

**1. Iniciar el servidor:**
```bash
cargo run --bin noah-server
```

**2. Ejecutar benchmark (en otra terminal):**
```bash
cargo run --bin noah-bench
```

## 📊 Rendimiento

### Resultados de Benchmarks

| Configuración | Throughput | P50 Latencia | P95 Latencia | P99 Latencia |
|---------------|------------|--------------|--------------|--------------|
| **Ligero** (10 hilos, 100 req) | 3,144 req/s | 2.74ms | 5.29ms | 7.86ms |
| **Intensivo** (200 hilos, 2K req) | 8,163 req/s | 23.33ms | 41.44ms | 51.74ms |
| **Solo GET** (50 hilos, 2K req) | 7,919 req/s | 6.04ms | 10.41ms | 13.38ms |
| **Extremo** (500 hilos, 1K req) | 7,777 req/s | 60.64ms | 110.53ms | 142.72ms |

### Comandos de Benchmark

```bash
# Benchmark ligero
cargo run --bin noah-bench -- --concurrency 10 --requests 100

# Benchmark intensivo
cargo run --bin noah-bench -- --concurrency 200 --requests 2000

# Solo operaciones SET
cargo run --bin noah-bench -- --operation set --requests 5000

# Solo operaciones GET
cargo run --bin noah-bench -- --operation get --concurrency 50 --requests 2000

# Benchmark extremo
cargo run --bin noah-bench -- --concurrency 500 --requests 1000
```

## 🛠️ API Reference

### Operaciones Disponibles

- **SET**: Almacenar clave-valor
- **GET**: Recuperar valor por clave
- **DELETE**: Eliminar clave
- **LIST**: Listar claves con prefijo

### Ejemplo de uso con gRPC

```rust
use noah_protocol::grpc::noah_service_client::NoahServiceClient;
use noah_protocol::grpc::SetRequest;

let mut client = NoahServiceClient::connect("http://127.0.0.1:50051").await?;
let request = tonic::Request::new(SetRequest {
    key: "mi_clave".to_string(),
    value: "mi_valor".to_string(),
});
let response = client.set(request).await?;
```

## 🔧 Configuración

### Servidor (noah-server)

- **Puerto**: 50051 (gRPC)
- **Storage**: DashMap (thread-safe HashMap)
- **Concurrencia**: Ilimitada (Tokio async)

### Cliente de Benchmark (noah-bench)

```bash
cargo run --bin noah-bench -- --help
```

**Opciones disponibles:**
- `--target`: Dirección del servidor (default: http://127.0.0.1:50051)
- `--concurrency`: Número de hilos concurrentes (default: 100)
- `--requests`: Operaciones por hilo (default: 1000)
- `--operation`: Tipo de operación - set, get, mixed (default: mixed)

## 🏗️ Desarrollo

### Estructura del Proyecto

```
nanoEcosytem/
├── Cargo.toml                 # Workspace configuration
├── noah-protocol/
│   ├── proto/noah.proto       # gRPC service definition
│   ├── build.rs              # Protocol Buffers compilation
│   └── src/lib.rs            # Shared types and errors
├── noah-server/
│   └── src/main.rs           # Database server implementation
└── noah-bench/
    └── src/main.rs           # Benchmarking tool
```

### Compilar

```bash
# Verificar que todo compila
cargo check

# Compilar en modo release
cargo build --release

# Ejecutar tests
cargo test
```

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

## 🙏 Reconocimientos

- [Tokio](https://tokio.rs/) - Runtime async para Rust
- [tonic](https://github.com/hyperium/tonic) - gRPC implementation
- [DashMap](https://github.com/xacrimon/dashmap) - Concurrent HashMap
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing

---

**⭐ Si este proyecto te resulta útil, ¡dale una estrella!**
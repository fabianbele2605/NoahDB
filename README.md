# 🚀 NoahDB Ecosystem

> **Ultra-fast in-memory database with professional benchmarking tools**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![gRPC](https://img.shields.io/badge/gRPC-Protocol%20Buffers-blue.svg)](https://grpc.io)
[![Crates.io](https://img.shields.io/crates/v/noah-protocol.svg)](https://crates.io/crates/noah-protocol)
[![Performance](https://img.shields.io/badge/Performance-4K%20req%2Fs-green.svg)](#-performance)

## 📋 Overview

NoahDB is a modular database ecosystem built in Rust, designed for maximum performance and scalability. It uses gRPC for high-speed communication and provides professional benchmarking tools.

### 🏗️ Architecture: Engine vs Service

```
┌─────────────────────────────────────────────────────────┐
│                    NoahDB Ecosystem                      │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ noah-protocol│  │ noah-server  │  │  noah-bench  │ │
│  │   (gRPC)     │  │  (Service)   │  │ (Benchmark)  │ │
│  └──────────────┘  └──────┬───────┘  └──────────────┘ │
│                            │                            │
│                     ┌──────▼───────┐                   │
│                     │  NanoEngine  │                   │
│                     │   (Adapter)  │                   │
│                     └──────┬───────┘                   │
│                            │                            │
│                     ┌──────▼───────┐                   │
│                     │   NanoDB     │                   │
│                     │   (Engine)   │                   │
│                     └──────────────┘                   │
└─────────────────────────────────────────────────────────┘
```

**NanoDB** = The Engine (V8 Motor)
- Pure database engine
- B-Trees, LSM, algorithms
- Persistence and indexing

**NoahDB** = The Service (Ferrari)
- Network protocols (HTTP + gRPC)
- APIs and tools
- Uses NanoDB as engine

### Components

- **`noah-protocol`**: Shared library defining gRPC APIs and data types (published on crates.io)
- **`noah-server`**: Database server with NanoDB engine integration
- **`noah-bench`**: Professional benchmarking client with P50/P95/P99 metrics

## ⚡ Features

- 🚀 **Ultra-fast**: Up to 4,000+ req/s under heavy load
- 🔒 **Thread-safe**: Massive concurrency without explicit locks
- 📊 **Professional metrics**: P50, P95, P99 latencies
- 🛠️ **Modular**: Cargo Workspace architecture
- 🌐 **gRPC**: High-performance communication
- ⚙️ **Configurable**: Flexible CLI for different workloads
- 📦 **Published**: Available on crates.io
- 🏗️ **Engine vs Service**: Professional architecture pattern

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Cargo

### Installation

```bash
git clone https://github.com/fabianbele2605/NoahDB.git
cd NoahDB
cargo build --release
```

### Usage

**1. Start the server:**
```bash
cargo run --bin noah-server
```

**2. Run benchmark (in another terminal):**
```bash
cargo run --bin noah-bench
```

## 📊 Performance

### Benchmark Results (with NanoDB Engine)

| Configuration | Throughput | P50 Latency | P95 Latency | P99 Latency |
|---------------|------------|-------------|-------------|-------------|
| **Light** (10 threads, 100 req) | 3,144 req/s | 2.74ms | 5.29ms | 7.86ms |
| **Heavy** (200 threads, 2K req) | 3,961 req/s | 23.09ms | 44.45ms | 65.22ms |
| **GET only** (50 threads, 2K req) | 7,919 req/s | 6.04ms | 10.41ms | 13.38ms |

### Benchmark Commands

```bash
# Light benchmark
cargo run --bin noah-bench -- --concurrency 10 --requests 100

# Heavy benchmark
cargo run --bin noah-bench -- --concurrency 200 --requests 2000

# SET operations only
cargo run --bin noah-bench -- --operation set --requests 5000

# GET operations only
cargo run --bin noah-bench -- --operation get --concurrency 50 --requests 2000


```

## 🛠️ API Reference

### Available Operations

- **SET**: Store key-value
- **GET**: Retrieve value by key
- **DELETE**: Delete key
- **LIST**: List keys with prefix

### Example with gRPC

```rust
use noah_protocol::grpc::noah_service_client::NoahServiceClient;
use noah_protocol::grpc::SetRequest;

let mut client = NoahServiceClient::connect("http://127.0.0.1:50051").await?;
let request = tonic::Request::new(SetRequest {
    key: "my_key".to_string(),
    value: "my_value".to_string(),
});
let response = client.set(request).await?;
```

### Using as Library

Add to your `Cargo.toml`:

```toml
[dependencies]
noah-protocol = "0.1.0"
```

## 🔧 Configuration

### Server (noah-server)

- **Port**: 50051 (gRPC), 8080 (HTTP)
- **Storage**: NanoDB engine (thread-safe)
- **Concurrency**: Unlimited (Tokio async)

### Benchmark Client (noah-bench)

```bash
cargo run --bin noah-bench -- --help
```

**Available options:**
- `--target`: Server address (default: http://127.0.0.1:50051)
- `--concurrency`: Number of concurrent threads (default: 100)
- `--requests`: Operations per thread (default: 1000)
- `--operation`: Operation type - set, get, mixed (default: mixed)

## 🏗️ Project Structure

```
NoahDB/
├── Cargo.toml                 # Workspace configuration
├── noah-protocol/             # gRPC definitions (published on crates.io)
│   ├── proto/noah.proto       # Protocol Buffers
│   ├── build.rs              # Code generation
│   └── src/lib.rs            # Shared types
├── noah-server/               # Database server
│   └── src/
│       ├── main.rs           # Server implementation
│       └── engine_adapter.rs # NanoDB integration
└── noah-bench/                # Benchmarking tool
    └── src/main.rs           # Benchmark implementation
```

### Build

```bash
# Check compilation
cargo check

# Build in release mode
cargo build --release

# Run tests
cargo test
```

## 🤝 Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [NoahDB-Dashboard](https://github.com/fabianbele2605/NoahDB-Dashboard) - Real-time visualization dashboard
- [NanoDB](https://github.com/fabianbele2605/arquitectura-hexagonal-nanodb) - Database engine core

## 🙏 Acknowledgments

- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [tonic](https://github.com/hyperium/tonic) - gRPC implementation
- [NanoDB](https://github.com/fabianbele2605/arquitectura-hexagonal-nanodb) - Database engine
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing

---

**⭐ If you find this project useful, give it a star!**
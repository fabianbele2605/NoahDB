# noah-protocol

[![Crates.io](https://img.shields.io/crates/v/noah-protocol.svg)](https://crates.io/crates/noah-protocol)
[![Documentation](https://docs.rs/noah-protocol/badge.svg)](https://docs.rs/noah-protocol)

Protocol definitions and gRPC services for NoahDB - Ultra-fast in-memory database.

## Features

- 🚀 gRPC service definitions
- 📦 Protocol Buffers schemas  
- 🔧 Shared types and errors
- ⚡ High-performance serialization

## Installation

```toml
[dependencies]
noah-protocol = "0.1"

use noah_protocol::grpc::noah_service_client::NoahServiceClient;
use noah_protocol::grpc::SetRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NoahServiceClient::connect("http://127.0.0.1:50051").await?;
    
    let request = tonic::Request::new(SetRequest {
        key: "my_key".to_string(),
        value: "my_value".to_string(),
    });
    
    let response = client.set(request).await?;
    println!("Success: {}", response.into_inner().success);
    
    Ok(())
}


## Related Projects
NoahDB - Database server

NoahDB-Dashboard - Real-time dashboard

## License

MIT

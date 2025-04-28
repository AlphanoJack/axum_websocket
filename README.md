# axum_websocket

Real-time message transmission system using Axum WebSocket and gRPC

## Project Overview

This project implements a real-time message transmission system using Axum WebSocket server and gRPC. Key features include:

- WebSocket client handshake and connection management
- Message reception via gRPC
- Transmission of received messages to connected WebSocket clients

## Tech Stack

- Rust
- Axum (WebSocket server)
- Tonic (gRPC)
- Tokio (Async runtime)

## Project Structure

```
src/
├── main.rs           # Main application entry point
├── websocket.rs      # WebSocket handler and logic
└── grpc.rs           # gRPC service implementation
```

## Getting Started

1. Clone the project
```bash
git clone [repository-url]
cd axum_websocket
```

2. Install dependencies
```bash
cargo build
```

3. Run the server
```bash
cargo run
```

## API Endpoints

- WebSocket connection: `ws://localhost:3000/ws`
- gRPC service: `localhost:50051`


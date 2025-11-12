# Architecture Documentation

## Overview

The Solana WebSocket Service is a Rust-based application that listens to Solana blockchain events and broadcasts them to connected WebSocket clients in real-time.

## Components

### Main Components

#### 1. Main Entry Point (`src/main.rs`)
- Initializes the application
- Sets up environment variables and logging
- Creates a broadcast channel for event distribution
- Spawns the RPC listener task
- Starts the WebSocket server

#### 2. RPC Listener (`src/rpc.rs`)
- Connects to Solana RPC WebSocket
- Subscribes to log events for the configured program
- Parses incoming events
- Broadcasts events to connected clients via the broadcast channel

#### 3. WebSocket Server (`src/server.rs`)
- Accepts incoming WebSocket connections
- Subscribes to the broadcast channel
- Sends events to all connected clients
- Handles client disconnections

#### 4. Types (`src/types.rs`)
- Defines data structures for events
- `TokenEvent`: Main event structure
- `TokenDetails`: Token information
- `PumpData`: Pump.fun specific data

### Alternative/Unused Components

#### 1. Solana Client (`src/solana_client.rs`)
- Alternative implementation of Solana WebSocket client
- Includes reconnection logic
- Not currently used in main.rs
- Can be used as a replacement for `rpc.rs` if needed

#### 2. WS Server (`src/ws_server.rs`)
- Alternative WebSocket server implementation
- Similar to `server.rs` but with different error handling
- Not currently used in main.rs
- Can be used as a replacement for `server.rs` if needed

#### 3. Pump Parser (`src/pump_parser.rs`)
- Placeholder for transaction parsing logic
- Currently returns mock data
- Should be implemented with actual transaction parsing
- Used by `solana_client.rs` but not by `rpc.rs`

## Data Flow

```
1. Solana RPC WebSocket
   ↓
2. RPC Listener (rpc.rs)
   ↓
3. Event Parsing
   ↓
4. Broadcast Channel
   ↓
5. WebSocket Server (server.rs)
   ↓
6. Connected Clients
```

## Event Structure

```rust
TokenEvent {
    event_type: String,
    timestamp: String,
    transaction_signature: String,
    token: TokenDetails {
        mint_address: String,
        name: String,
        symbol: String,
        creator: String,
        supply: u64,
        decimals: u8,
    },
    pump_data: PumpData {
        bonding_curve: String,
        virtual_sol_reserves: u64,
        virtual_token_reserves: u64,
    },
}
```

## Configuration

- **Environment Variables**: Loaded from `.env` file
- **RPC WebSocket URL**: Configured via `RPC_WS` environment variable
- **Server Port**: Configured via `SERVER_PORT` environment variable
- **Logging**: Configured via `RUST_LOG` environment variable

## Future Improvements

1. **Transaction Parsing**: Implement actual transaction parsing in `pump_parser.rs`
2. **Error Handling**: Improve error handling and reconnection logic
3. **Metrics**: Add metrics and monitoring
4. **Authentication**: Add authentication for WebSocket connections
5. **Rate Limiting**: Add rate limiting for clients
6. **Message Filtering**: Add message filtering based on client preferences
7. **Health Checks**: Add health check endpoints
8. **Load Balancing**: Support multiple RPC connections for load balancing

## Testing

- Unit tests should be added in `tests/` directory
- Integration tests for WebSocket connections
- Mock RPC server for testing

## Deployment

- Build with `cargo build --release`
- Run with environment variables set
- Consider using systemd or similar for process management
- Use reverse proxy (nginx) for production WebSocket connections


# Solana WebSocket Service

A high-performance Rust-based WebSocket service that listens to Solana blockchain events (specifically Pump.fun token creation events) and broadcasts them to connected clients in real-time.

## 📋 Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Code Structure](#code-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Documentation](#api-documentation)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## 🚀 Features

- **Real-time Event Streaming**: Listens to Solana RPC WebSocket for blockchain events
- **WebSocket Server**: Broadcasts events to multiple connected clients
- **Pump.fun Integration**: Monitors token creation events on Pump.fun
- **High Performance**: Built with Rust and Tokio for async/await concurrency
- **Automatic Reconnection**: Handles connection failures gracefully
- **Structured Events**: JSON-formatted event data with token and pump data

## 🏗️ Architecture

The service consists of two main components:

1. **RPC Listener** (`src/rpc.rs`): Connects to Solana RPC WebSocket and subscribes to log events
2. **WebSocket Server** (`src/server.rs`): Accepts client connections and broadcasts events

```
┌─────────────────┐         ┌──────────────┐         ┌─────────────────┐
│  Solana RPC WS  │────────▶│  RPC Listener│────────▶│ Broadcast Channel│
└─────────────────┘         └──────────────┘         └─────────────────┘
                                                                 │
                                                                 ▼
┌─────────────────┐         ┌──────────────┐         ┌─────────────────┐
│  WebSocket      │◀────────│  WS Server   │◀────────│   Clients       │
│  Clients        │         └──────────────┘         └─────────────────┘
└─────────────────┘
```

## 📁 Code Structure

### Source Files

#### `src/main.rs`
The application entry point that orchestrates the entire service.

**Key Responsibilities:**
- Loads environment variables using `dotenv`
- Initializes the logger with `env_logger`
- Creates a broadcast channel for event distribution (capacity: 100)
- Spawns the RPC listener as a separate async task
- Starts the WebSocket server (blocking operation)

**Code Flow:**
```rust
1. Initialize environment and logging
2. Parse SERVER_PORT from environment (default: 9000)
3. Create broadcast channel (tx, rx)
4. Spawn RPC listener task with tx.clone()
5. Start WebSocket server with rx
```

#### `src/rpc.rs`
Handles the connection to Solana RPC WebSocket and event subscription.

**Key Functions:**
- `start_rpc_listener(tx: Sender<String>)`: Main entry point that runs in a loop with automatic reconnection
- `connect_and_listen(ws_url, program_id, tx)`: Establishes WebSocket connection and processes messages

**Implementation Details:**
- Uses `tokio-tungstenite` for WebSocket connections
- Subscribes to `logsSubscribe` RPC method with program ID filter
- Filters events by checking for "initialize_token" in logs
- Parses events using `parse_rpc_event()` from parser module
- Broadcasts parsed events via the channel
- Implements automatic reconnection with 5-second delay on errors

**RPC Subscription Format:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "logsSubscribe",
  "params": [
    { "mentions": [program_id] },
    { "commitment": "finalized" }
  ]
}
```

#### `src/server.rs`
Manages the WebSocket server that broadcasts events to clients.

**Key Functions:**
- `start_websocket_server(rx: Receiver<String>, port: u16)`: Main server function

**Implementation Details:**
- Binds to `0.0.0.0:{port}` to accept connections from any interface
- Uses `tokio::net::TcpListener` for accepting TCP connections
- Each client connection is handled in a separate spawned task
- Uses `tokio-tungstenite::accept_async` to upgrade TCP to WebSocket
- Each client gets its own receiver via `rx.resubscribe()`
- Sends events as text messages to all connected clients
- Handles client disconnections gracefully

**Connection Handling:**
- New connections are logged with peer address
- Each client runs in its own async task
- Messages are sent as `tungstenite::Message::Text`
- Disconnections are detected when send fails

#### `src/parser.rs`
Contains parsing logic for RPC events and transaction data.

**Key Functions:**
- `parse_rpc_event(event_json: &Value) -> Option<TokenEvent>`: Parses Solana RPC JSON response
- `parse_pump_tx(signature: &str, raw_log: &str) -> Option<TokenEvent>`: Parses pump transaction (placeholder)

**Implementation Details:**
- Extracts transaction signature from RPC response: `params.result.value.signature`
- Extracts logs array from: `params.result.value.logs`
- Filters for token creation events by checking for "initialize_token" in logs
- Currently uses mock data (TODO: implement actual transaction parsing)
- Returns `None` if event doesn't match expected format

**Event Filtering:**
```rust
logs.iter().any(|l| {
    l.as_str()
        .unwrap_or("")
        .contains("initialize_token")
})
```

#### `src/types.rs`
Defines all data structures used throughout the application.

**Structures:**
- `TokenEvent`: Main event structure containing all token creation information
  - `event_type`: String identifier (e.g., "token_created")
  - `timestamp`: ISO 8601 formatted timestamp
  - `transaction_signature`: Solana transaction signature
  - `token`: TokenDetails struct
  - `pump_data`: PumpData struct

- `TokenDetails`: Token-specific information
  - `mint_address`: Token mint address
  - `name`: Token name
  - `symbol`: Token symbol
  - `creator`: Creator public key
  - `supply`: Total token supply
  - `decimals`: Token decimals

- `PumpData`: Pump.fun specific data
  - `bonding_curve`: Bonding curve address
  - `virtual_sol_reserves`: Virtual SOL reserves
  - `virtual_token_reserves`: Virtual token reserves

All structures implement `Serialize`, `Deserialize`, and `Debug` traits for JSON serialization.

#### `src/error.rs`
Defines custom error types for the application.

**Error Types:**
- `AppError::ConfigError(String)`: Configuration-related errors
- `AppError::ConnectionError(String)`: WebSocket connection errors
- `AppError::RpcError(String)`: RPC-related errors
- `AppError::ParseError(String)`: Parsing errors
- `AppError::ServerError(String)`: Server-related errors

**Type Alias:**
- `AppResult<T>`: Convenience alias for `Result<T, AppError>`

Currently defined but not extensively used in the codebase (available for future error handling improvements).

### Archive Files (`src/archive/`)

The archive directory contains alternative implementations that are not currently used:

- `pump_parser.rs`: Alternative parser implementation
- `solana_client.rs`: Alternative Solana client with reconnection logic
- `ws_server.rs`: Alternative WebSocket server implementation

These files are kept for reference and can be used as alternatives or for future development.

## 📦 Prerequisites

- **Rust** (latest stable version): [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)
- **Solana RPC WebSocket URL** (can use public endpoints or your own RPC provider)

## 🔧 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourname/solana-ws.git
cd solana-ws
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Configure Environment Variables

Create a `.env` file in the root directory:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
SERVER_PORT=9000
RPC_WS=wss://api.mainnet-beta.solana.com
RUST_LOG=info
PROGRAM_ID=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SERVER_PORT` | Port for WebSocket server | `9000` | No |
| `RPC_WS` | Solana RPC WebSocket URL | - | Yes |
| `RUST_LOG` | Logging level (trace, debug, info, warn, error) | `info` | No |
| `PROGRAM_ID` | Program ID to monitor (Pump.fun program) | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` | No |

### RPC WebSocket URLs

- **Mainnet**: `wss://api.mainnet-beta.solana.com`
- **Devnet**: `wss://api.devnet.solana.com`
- **Testnet**: `wss://api.testnet.solana.com`

For production, consider using a dedicated RPC provider like:
- Helius
- QuickNode
- Triton

## 🎯 Usage

### Run the Service

```bash
cargo run
```

Or with release build:

```bash
cargo run --release
```

The service will:
1. Connect to the Solana RPC WebSocket
2. Subscribe to log events for the configured program
3. Start the WebSocket server on the configured port
4. Broadcast events to all connected clients

### Connect as a Client

#### Python Example

```python
import asyncio
import websockets
import json

async def listen():
    uri = "ws://localhost:9000"
    async with websockets.connect(uri) as websocket:
        print("Connected to WebSocket server...")
        try:
            while True:
                msg = await websocket.recv()
                event = json.loads(msg)
                print("Received:", json.dumps(event, indent=2))
        except websockets.ConnectionClosed:
            print("Connection closed")

asyncio.run(listen())
```

#### JavaScript Example

```javascript
const ws = new WebSocket('ws://localhost:9000');

ws.onopen = () => {
    console.log('Connected to WebSocket server');
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Received:', data);
};

ws.onerror = (error) => {
    console.error('WebSocket error:', error);
};

ws.onclose = () => {
    console.log('Connection closed');
};
```

## 📚 API Documentation

### Event Format

Events are broadcast as JSON strings with the following structure:

```json
{
  "event_type": "token_created",
  "timestamp": "2024-01-01T00:00:00Z",
  "transaction_signature": "5j7s8K9...",
  "token": {
    "mint_address": "ABC123...",
    "name": "ExampleToken",
    "symbol": "EXT",
    "creator": "CreatorPubKey...",
    "supply": 1000000000,
    "decimals": 6
  },
  "pump_data": {
    "bonding_curve": "BondingCurveAddr...",
    "virtual_sol_reserves": 30000000000,
    "virtual_token_reserves": 1073000000000000
  }
}
```

### WebSocket Connection

- **Protocol**: WebSocket (ws:// or wss://)
- **Endpoint**: `ws://localhost:{SERVER_PORT}`
- **Message Format**: JSON string
- **Connection**: Persistent, unidirectional (server → client)
- **Reconnection**: Clients should implement reconnection logic

## 🔨 Development

### Adding New Features

1. **New Event Types**: Extend `src/types.rs` with new structures
2. **Parsing Logic**: Implement actual transaction parsing in `src/parser.rs`
3. **Error Handling**: Use `src/error.rs` for structured error handling
4. **Tests**: Write unit tests in `tests/` directory

### Code Organization

- **Main Module** (`main.rs`): Orchestrates the application
- **RPC Module** (`rpc.rs`): Handles Solana RPC WebSocket connection
- **Server Module** (`server.rs`): Manages WebSocket server and client connections
- **Types Module** (`types.rs`): Defines data structures
- **Parser Module** (`parser.rs`): Parses transaction data (needs implementation)
- **Error Module** (`error.rs`): Custom error types

### Building for Production

```bash
cargo build --release
```

The binary will be located at `target/release/apeing-ws-service` (or `.exe` on Windows).

### Code Quality

Run clippy for linting:

```bash
cargo clippy -- -D warnings
```

Format code:

```bash
cargo fmt
```

## 🧪 Testing

### Run Tests

```bash
cargo test
```

### Test WebSocket Connection

Use a WebSocket client to connect to the server and verify events are received.

## 🐛 Troubleshooting

### Connection Issues

- **RPC WebSocket fails to connect**: Check your `RPC_WS` URL and network connectivity
- **WebSocket server fails to start**: Check if the port is already in use
- **No events received**: Verify the program ID is correct and events are being generated

### Logging

Set `RUST_LOG` environment variable for detailed logging:

```bash
RUST_LOG=debug cargo run
```

### Common Errors

1. **"WebSocket connection failed"**: Invalid RPC URL or network issue
2. **"Failed to bind"**: Port already in use, change `SERVER_PORT`
3. **"Environment variable not found"**: Create `.env` file with required variables

## 📂 Project Structure

```
solana-ws/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── rpc.rs             # Solana RPC WebSocket listener
│   ├── server.rs          # WebSocket server implementation
│   ├── parser.rs          # Event parsing logic
│   ├── types.rs           # Data structures and types
│   ├── error.rs           # Custom error types
│   └── archive/           # Archived/unused code
│       ├── pump_parser.rs
│       ├── solana_client.rs
│       └── ws_server.rs
├── tests/                 # Integration tests
│   └── README.md
├── config/                # Configuration documentation
│   └── README.md
├── docs/                  # Documentation
│   ├── ARCHITECTURE.md
│   └── PROJECT_STRUCTURE.md
├── Cargo.toml             # Rust dependencies
├── Cargo.lock             # Dependency lock file
├── .env.example           # Environment variables template
├── .gitignore             # Git ignore rules
└── README.md              # This file
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Solana Foundation for the blockchain infrastructure
- Tokio team for the async runtime
- Tungstenite for WebSocket implementation

## 📞 Support

For issues and questions:
- Open an issue on GitHub
- Check the documentation in `docs/`
- Review the code comments for implementation details

---

**Note**: This service currently uses placeholder data for token parsing. Implement actual transaction parsing logic in `src/parser.rs` for production use. See the `parse_pump_tx` function for the TODO.

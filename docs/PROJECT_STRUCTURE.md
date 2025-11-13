# Project Structure

## Overview

This document describes the standard folder structure of the Solana WebSocket Service project.

## Directory Structure

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
│       ├── pump_parser.rs # Alternative parser (unused)
│       ├── solana_client.rs # Alternative client (unused)
│       └── ws_server.rs   # Alternative server (unused)
├── examples/              # Example clients
│   ├── client.py          # Python WebSocket client
│   ├── client.js          # Node.js WebSocket client
│   ├── requirements.txt   # Python dependencies
│   ├── package.json       # Node.js dependencies
│   └── README.md          # Examples documentation
├── tests/                 # Unit and integration tests
│   └── README.md          # Test documentation
├── config/                # Configuration files
│   └── README.md          # Configuration documentation
├── docs/                  # Documentation
│   ├── ARCHITECTURE.md    # Architecture documentation
│   └── PROJECT_STRUCTURE.md  # This file
├── Cargo.toml             # Rust dependencies
├── Cargo.lock             # Dependency lock file
├── .env.example           # Environment variables template
├── .gitignore             # Git ignore rules
├── LICENSE                # MIT License
├── README.md              # Main documentation
├── CONTRIBUTING.md        # Contribution guidelines
├── CHANGELOG.md           # Project changelog
└── .github/               # GitHub configuration
    └── workflows/         # CI/CD workflows
        ├── ci.yml         # Continuous Integration
        └── rust.yml       # Rust-specific CI
```

## File Descriptions

### Source Code (`src/`)

- **main.rs**: Application entry point that initializes the service
- **rpc.rs**: Connects to Solana RPC WebSocket and listens for events
- **server.rs**: WebSocket server that broadcasts events to clients
- **parser.rs**: Parses RPC events and transaction data
- **types.rs**: Data structures for events and tokens
- **error.rs**: Custom error types for the application
- **archive/**: Contains alternative implementations (not currently used)
  - **pump_parser.rs**: Alternative parser implementation
  - **solana_client.rs**: Alternative Solana client with reconnection logic
  - **ws_server.rs**: Alternative WebSocket server implementation

### Examples (`examples/`)

- **client.py**: Python WebSocket client example
- **client.js**: Node.js WebSocket client example
- **requirements.txt**: Python dependencies for examples
- **package.json**: Node.js dependencies for examples
- **README.md**: Documentation for examples

### Tests (`tests/`)

- **README.md**: Documentation for tests
- Unit tests should be added here
- Integration tests should be added here

### Configuration (`config/`)

- **README.md**: Configuration documentation
- Configuration files (future)

### Documentation (`docs/`)

- **ARCHITECTURE.md**: Architecture documentation
- **PROJECT_STRUCTURE.md**: This file

### Root Files

- **Cargo.toml**: Rust project configuration and dependencies
- **Cargo.lock**: Dependency lock file (should be committed for applications)
- **.env.example**: Environment variables template
- **.gitignore**: Git ignore rules
- **LICENSE**: MIT License file
- **README.md**: Main project documentation
- **CONTRIBUTING.md**: Guidelines for contributors
- **CHANGELOG.md**: Project version history and changes

### GitHub Configuration (`.github/`)

- **workflows/ci.yml**: Continuous Integration workflow
- **workflows/rust.yml**: Rust-specific CI workflow

## Standard Practices

### Environment Variables

- Create `.env` file from `.env.example` for local development
- Never commit `.env` file (it's in `.gitignore`)
- Always commit `.env.example` with template values

### Dependencies

- All Rust dependencies are in `Cargo.toml`
- Python dependencies are in `examples/requirements.txt`
- Node.js dependencies are in `examples/package.json`
- Lock files should be committed for applications

### Documentation

- Main documentation in `README.md`
- Architecture documentation in `docs/ARCHITECTURE.md`
- Code comments in source files
- Example documentation in `examples/README.md`

### Testing

- Unit tests in `tests/` directory
- Integration tests in `tests/` directory
- Example clients in `examples/` directory

## File Naming Conventions

- **Rust files**: `snake_case.rs`
- **Python files**: `snake_case.py`
- **Documentation**: `UPPERCASE.md`
- **Configuration**: `lowercase` or `UPPERCASE` depending on format
- **Environment files**: `.env.example` (with dot prefix)

## Git Ignore Rules

- `/target/`: Rust build artifacts
- `*.rs.bk`: Rust backup files
- `.env`: Environment variables (sensitive)
- `.env.local`: Local environment variables
- `.env.*.local`: Local environment variable variants
- IDE files (`.vscode/`, `.idea/`, etc.)
- OS files (`.DS_Store`, `Thumbs.db`)
- Log files (`*.log`, `logs/`)
- Build artifacts (`*.exe`, `*.dll`, etc.)

## Future Structure

- **scripts/**: Utility scripts
- **.github/**: GitHub workflows and templates
- **docker/**: Docker configuration files
- **deploy/**: Deployment configurations


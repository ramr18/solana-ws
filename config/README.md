# Configuration

This directory contains configuration files for the Solana WebSocket Service.

## Configuration Files

- `.env.example` - Example environment variables (located in project root)
- `config.toml` - Optional configuration file (future)

## Environment Variables

See `.env.example` in the project root for available environment variables.

## Configuration Examples

### Development

```env
SERVER_PORT=9000
RPC_WS=wss://api.devnet.solana.com
RUST_LOG=debug
```

### Production

```env
SERVER_PORT=9000
RPC_WS=wss://your-rpc-provider.com
RUST_LOG=info
```

## RPC Providers

- **Mainnet**: `wss://api.mainnet-beta.solana.com`
- **Devnet**: `wss://api.devnet.solana.com`
- **Testnet**: `wss://api.testnet.solana.com`

For production, consider using a dedicated RPC provider:
- Helius
- QuickNode
- Triton


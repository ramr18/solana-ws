# Tests

This directory contains unit tests and integration tests for the Solana WebSocket Service.

## Running Tests

```bash
cargo test
```

## Test Structure

- Unit tests for individual modules
- Integration tests for WebSocket connections
- Mock RPC server for testing

## Adding Tests

1. Create test files in this directory
2. Use `#[cfg(test)]` for unit tests
3. Use integration tests for end-to-end testing

## Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_event_serialization() {
        let event = TokenEvent {
            event_type: "token_created".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            transaction_signature: "test".to_string(),
            token: TokenDetails {
                mint_address: "test".to_string(),
                name: "Test".to_string(),
                symbol: "TST".to_string(),
                creator: "test".to_string(),
                supply: 1000000,
                decimals: 6,
            },
            pump_data: PumpData {
                bonding_curve: "test".to_string(),
                virtual_sol_reserves: 1000000,
                virtual_token_reserves: 1000000,
            },
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("token_created"));
    }
}
```


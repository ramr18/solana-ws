# Contributing to Solana WebSocket Service

Thank you for your interest in contributing to the Solana WebSocket Service! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/solana-ws.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `cargo test`
6. Commit your changes: `git commit -m 'Add some feature'`
7. Push to your branch: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

### Code Quality

Before submitting, please ensure:

1. **Formatting**: Run `cargo fmt` to format your code
2. **Linting**: Run `cargo clippy -- -D warnings` to check for linting issues
3. **Tests**: All tests pass with `cargo test`
4. **Documentation**: Add documentation for new public APIs

## Code Style

- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and small
- Handle errors appropriately

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in imperative mood (e.g., "Add", "Fix", "Update")
- Reference issues when applicable (e.g., "Fix #123")

## Pull Request Process

1. Ensure your code follows the project's style guidelines
2. Update documentation if needed
3. Add tests for new features
4. Ensure all tests pass
5. Update CHANGELOG.md if applicable
6. Request review from maintainers

## Reporting Issues

When reporting issues, please include:

- Description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Relevant logs or error messages

## Feature Requests

For feature requests, please:

- Describe the feature clearly
- Explain the use case
- Discuss potential implementation approaches
- Consider backwards compatibility

## Questions?

Feel free to open an issue for questions or discussions.

Thank you for contributing!



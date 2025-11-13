# Examples

This directory contains example clients for connecting to the Solana WebSocket Service.

## Python Client

### Requirements

Install the required Python package:

```bash
pip install websockets
```

Or use the requirements file:

```bash
pip install -r requirements.txt
```

### Usage

```bash
python client.py
```

The client will connect to `ws://localhost:9000` by default. You can modify the `WS_URL` variable in the script to change the connection endpoint.

## JavaScript/Node.js Client

### Requirements

Install the required Node.js package:

```bash
npm install ws
```

### Usage

```bash
node client.js
```

The client will connect to `ws://localhost:9000` by default. You can modify the `WS_URL` constant in the script to change the connection endpoint.

## Features

Both example clients:
- Connect to the WebSocket server
- Listen for token creation events
- Parse and display event data
- Handle connection errors and reconnections
- Provide formatted output


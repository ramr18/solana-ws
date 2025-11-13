#!/usr/bin/env python3
"""
Example Python client for Solana WebSocket Service

This client connects to the WebSocket server and listens for token creation events.
"""

import asyncio
import json
import websockets
from datetime import datetime

# WebSocket server URL
WS_URL = "ws://localhost:9000"


async def listen_to_events():
    """Connect to WebSocket server and listen for events."""
    print(f"Connecting to {WS_URL}...")
    
    try:
        async with websockets.connect(WS_URL) as websocket:
            print("✓ Connected to WebSocket server")
            print("Waiting for token creation events...\n")
            
            try:
                async for message in websocket:
                    try:
                        event = json.loads(message)
                        display_event(event)
                    except json.JSONDecodeError as e:
                        print(f"Error parsing JSON: {e}")
                        print(f"Raw message: {message}")
            except websockets.ConnectionClosed:
                print("\n✗ Connection closed by server")
            except Exception as e:
                print(f"\n✗ Error receiving message: {e}")
                
    except websockets.InvalidURI:
        print(f"✗ Invalid WebSocket URL: {WS_URL}")
    except ConnectionRefusedError:
        print(f"✗ Connection refused. Is the server running on {WS_URL}?")
    except Exception as e:
        print(f"✗ Connection error: {e}")


def display_event(event: dict):
    """Display a formatted token event."""
    print("=" * 80)
    print(f"Event Type: {event.get('event_type', 'N/A')}")
    print(f"Timestamp: {event.get('timestamp', 'N/A')}")
    print(f"Transaction: {event.get('transaction_signature', 'N/A')}")
    
    token = event.get('token', {})
    if token:
        print("\nToken Details:")
        print(f"  Mint Address: {token.get('mint_address', 'N/A')}")
        print(f"  Name: {token.get('name', 'N/A')}")
        print(f"  Symbol: {token.get('symbol', 'N/A')}")
        print(f"  Creator: {token.get('creator', 'N/A')}")
        print(f"  Supply: {token.get('supply', 'N/A'):,}")
        print(f"  Decimals: {token.get('decimals', 'N/A')}")
    
    pump_data = event.get('pump_data', {})
    if pump_data:
        print("\nPump Data:")
        print(f"  Bonding Curve: {pump_data.get('bonding_curve', 'N/A')}")
        print(f"  Virtual SOL Reserves: {pump_data.get('virtual_sol_reserves', 'N/A'):,}")
        print(f"  Virtual Token Reserves: {pump_data.get('virtual_token_reserves', 'N/A'):,}")
    
    print("=" * 80)
    print()


if __name__ == "__main__":
    try:
        asyncio.run(listen_to_events())
    except KeyboardInterrupt:
        print("\n\nClient stopped by user")
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")


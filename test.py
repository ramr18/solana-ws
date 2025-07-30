import asyncio
import websockets

async def listen():
    uri = "ws://localhost:9000"
    async with websockets.connect(uri) as websocket:
        print("Connected to WebSocket server...")
        try:
            while True:
                msg = await websocket.recv()
                print("Received:", msg)
        except websockets.ConnectionClosed:
            print("Connection closed")

asyncio.run(listen())

/**
 * Example Node.js client for Solana WebSocket Service
 * 
 * This client connects to the WebSocket server and listens for token creation events.
 * 
 * Usage:
 *   npm install ws
 *   node client.js
 */

const WebSocket = require('ws');

// WebSocket server URL
const WS_URL = 'ws://localhost:9000';

// Create WebSocket connection
const ws = new WebSocket(WS_URL);

ws.on('open', () => {
    console.log(`✓ Connected to WebSocket server at ${WS_URL}`);
    console.log('Waiting for token creation events...\n');
});

ws.on('message', (data) => {
    try {
        const event = JSON.parse(data.toString());
        displayEvent(event);
    } catch (error) {
        console.error('Error parsing JSON:', error.message);
        console.error('Raw message:', data.toString());
    }
});

ws.on('error', (error) => {
    if (error.code === 'ECONNREFUSED') {
        console.error(`✗ Connection refused. Is the server running on ${WS_URL}?`);
    } else {
        console.error('✗ WebSocket error:', error.message);
    }
});

ws.on('close', () => {
    console.log('\n✗ Connection closed');
});

/**
 * Display a formatted token event
 */
function displayEvent(event) {
    console.log('='.repeat(80));
    console.log(`Event Type: ${event.event_type || 'N/A'}`);
    console.log(`Timestamp: ${event.timestamp || 'N/A'}`);
    console.log(`Transaction: ${event.transaction_signature || 'N/A'}`);
    
    if (event.token) {
        console.log('\nToken Details:');
        console.log(`  Mint Address: ${event.token.mint_address || 'N/A'}`);
        console.log(`  Name: ${event.token.name || 'N/A'}`);
        console.log(`  Symbol: ${event.token.symbol || 'N/A'}`);
        console.log(`  Creator: ${event.token.creator || 'N/A'}`);
        console.log(`  Supply: ${event.token.supply ? event.token.supply.toLocaleString() : 'N/A'}`);
        console.log(`  Decimals: ${event.token.decimals || 'N/A'}`);
    }
    
    if (event.pump_data) {
        console.log('\nPump Data:');
        console.log(`  Bonding Curve: ${event.pump_data.bonding_curve || 'N/A'}`);
        console.log(`  Virtual SOL Reserves: ${event.pump_data.virtual_sol_reserves ? event.pump_data.virtual_sol_reserves.toLocaleString() : 'N/A'}`);
        console.log(`  Virtual Token Reserves: ${event.pump_data.virtual_token_reserves ? event.pump_data.virtual_token_reserves.toLocaleString() : 'N/A'}`);
    }
    
    console.log('='.repeat(80));
    console.log();
}

// Handle graceful shutdown
process.on('SIGINT', () => {
    console.log('\n\nClient stopped by user');
    ws.close();
    process.exit(0);
});


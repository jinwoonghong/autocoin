// Simple WebSocket test script
console.log('Testing WebSocket connection...');

// Create WebSocket connection - use hardcoded localhost for testing
const WS_URL = 'ws://127.0.0.1:8080/ws';

const ws = new WebSocket(WS_URL);

ws.onopen = function(event) {
    console.log('✅ WebSocket connected successfully!');
    console.log('Connection opened:', event);

    // Send a test message
    const testMessage = {
        type: 'test',
        data: {
            message: 'Hello from client',
            timestamp: new Date().toISOString()
        }
    };
    ws.send(JSON.stringify(testMessage));
    console.log('Sent test message:', testMessage);
};

ws.onmessage = function(event) {
    console.log('📨 Received message:', event.data);

    // Close connection after receiving a message
    setTimeout(() => {
        console.log('Closing connection...');
        ws.close();
    }, 1000);
};

ws.onclose = function(event) {
    if (event.wasClean) {
        console.log('✅ Connection closed cleanly, code:', event.code, 'reason:', event.reason);
    } else {
        console.error('❌ Connection died unexpectedly');
    }
};

ws.onerror = function(error) {
    console.error('❌ WebSocket error:', error);
    console.error('Error type:', error.type);
    console.error('Error message:', error.message);
};

// Timeout after 5 seconds if connection doesn't open
setTimeout(() => {
    if (ws.readyState !== WebSocket.OPEN) {
        console.error('❌ Connection timeout after 5 seconds');
        ws.close();
    }
}, 5000);
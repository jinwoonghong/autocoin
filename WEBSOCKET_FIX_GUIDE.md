# WebSocket Connection Fix Guide

## Summary of Issues Fixed

1. **Host Binding Issue**: Changed web server from binding to `127.0.0.1` to `0.0.0.0`
2. **Frontend WebSocket URL**: Updated to use proper WebSocket protocol (ws/wss)
3. **Enhanced Logging**: Added detailed logging for WebSocket connections
4. **Test Scripts**: Created test scripts to verify the fix

## Files Modified

### 1. `src/web/server.rs`
- Changed default host from `"127.0.0.1"` to `"0.0.0.0"`

### 2. `src/main.rs`
- Changed default CLI host from `"127.0.0.1"` to `"0.0.0.0"`

### 3. `web/dashboard.html`
- Updated WebSocket URL to use proper protocol:
  ```javascript
  const WS_URL = (window.location.protocol === 'https:' ? 'wss:' : 'ws:') + '//' + window.location.hostname + ':8080/ws';
  ```

### 4. `src/web/websocket.rs`
- Added detailed logging for WebSocket connection events

## Testing the Fix

### Method 1: Using Build Script
1. Run the build script:
   ```
   build_and_test.bat
   ```

### Method 2: Manual Testing
1. Start the server:
   ```
   cargo run --release
   ```

2. Test WebSocket connection:
   ```
   node web_test.js
   ```

3. Test HTTP endpoints:
   ```
   node http_test.js
   ```

4. Open dashboard in browser:
   ```
   http://localhost:8080/dashboard
   ```

## Expected Behavior

After the fix:
- ✅ WebSocket should connect successfully
- ✅ Connection status should show "Connected"
- ✅ Real-time updates should work (market prices, positions, etc.)
- ✅ Console should show WebSocket connection logs

## Common Issues and Solutions

### Issue: Connection Refused
- **Cause**: Server not running
- **Solution**: Ensure the server is started before opening the dashboard

### Issue: WebSocket Not Connected
- **Cause**: Firewall blocking port 8080
- **Solution**: Check firewall settings, allow port 8080

### Issue: CORS Error
- **Cause**: Browser blocking cross-origin requests
- **Solution**: The server already has CORS enabled for all origins

## Troubleshooting

1. **Check if server is running**:
   - Open http://localhost:8080/health in browser
   - Should return JSON response

2. **Check browser console**:
   - Open developer tools (F12)
   - Look for WebSocket errors
   - Check Network tab for WebSocket connection

3. **Check server logs**:
   - Look for "WebSocket connection established" message
   - Look for any error messages

## Alternative Connection URLs

If the main connection doesn't work, try these alternative URLs in your browser:
- `http://127.0.0.1:8080/dashboard`
- `http://localhost:8080/dashboard`
- `http://[your-ip]:8080/dashboard` (replace with your actual IP)

The WebSocket URLs would be:
- `ws://127.0.0.1:8080/ws`
- `ws://localhost:8080/ws`
- `ws://[your-ip]:8080/ws`
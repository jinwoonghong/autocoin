# AutoCoin Web Dashboard

A modern, real-time web-based dashboard for monitoring your AutoCoin trading bot.

## Features

- **Dark Theme**: Trading-appropriate dark interface
- **Real-time Updates**: WebSocket connection for live data
- **Agent Status**: Monitor all trading agents in real-time
- **Market Prices**: Track price changes for monitored markets
- **Balance & Position**: View current balance and open positions
- **Notifications**: Live feed of trading signals and events
- **Responsive Design**: Works on desktop and mobile devices

## Accessing the Dashboard

### Starting the Web Server

By default, the web server starts automatically when you run `autocoin`:

```bash
autocoin
```

The dashboard will be available at `http://127.0.0.1:8080` and your browser should open automatically.

### Command Line Options

```bash
# Start web server on custom host/port
autocoin --host 0.0.0.0 --port 3000

# Disable browser auto-open
autocoin --no-open-browser

# Disable web server entirely (use TUI instead)
autocoin --dashboard --no-web

# Daemon mode (no UI)
autocoin --daemon
```

### Full CLI Options

| Option | Default | Description |
|--------|---------|-------------|
| `--host` | 127.0.0.1 | Web server host address |
| `--port` | 8080 | Web server port |
| `--no-web` | false | Disable web server |
| `--no-open-browser` | false | Don't auto-open browser |
| `--dashboard` | false | Enable TUI dashboard instead |
| `--daemon` | false | Run in daemon mode (no UI) |

## Dashboard Sections

### Balance Card
- Total asset value (KRW)
- Available balance
- Today's PnL

### Position Card
- Current market
- Entry and current prices
- Unrealized PnL (amount and percentage)

### Agents Card
- Status of all trading agents
- Real-time state updates
- Connection status indicators

### Market Prices
- Live price updates via WebSocket
- 24-hour change percentage
- Trading volume

### Notifications
- Trading signals
- Order execution confirmations
- Error messages and warnings
- Time-stamped events

## WebSocket API

The dashboard uses WebSocket for real-time updates. Connect to:

```
ws://127.0.0.1:8080/ws
```

### Message Types

- `priceUpdate`: Market price changes
- `positionUpdate`: Position changes
- `agentStatus`: Agent state changes
- `balanceUpdate`: Balance changes
- `notification`: New notifications
- `systemStatus`: System status updates

## HTTP API

### Dashboard Data
```
GET /api/dashboard
```

Returns all dashboard data in a single response.

### Individual Endpoints
- `GET /api/status` - System status
- `GET /api/balance` - Balance information
- `GET /api/position` - Current position
- `GET /api/markets` - Market prices
- `GET /api/agents` - Agent states
- `GET /api/notifications` - Recent notifications
- `GET /api/trades` - Trade history

## Development

### File Structure
```
web/
├── dashboard.html    # Main dashboard HTML (served by Rust backend)
└── README.md         # This file

src/web/
├── handlers.rs       # HTTP request handlers
├── routes.rs         # Route configuration
├── server.rs         # Web server implementation
├── state.rs          # Shared state management
└── websocket.rs      # WebSocket handler
```

### Adding New Features

1. Add the data structure to `state.rs`
2. Create an API handler in `handlers.rs`
3. Register the route in `routes.rs`
4. Update the WebSocket message types if needed
5. Update the dashboard HTML to display the new data

## Troubleshooting

### WebSocket Not Connecting
- Check if the web server is running
- Verify the port is not blocked by firewall
- Check browser console for errors

### Data Not Updating
- Ensure trading agents are running
- Check the browser's WebSocket connection status
- View server logs for errors

### Cannot Access Dashboard
- Verify `--no-web` flag is not set
- Check the host and port settings
- Ensure the server started successfully

## Security

- The dashboard binds to `127.0.0.1` by default (localhost only)
- To access from other devices, use `--host 0.0.0.0`
- Consider adding authentication for production deployments
- Use HTTPS in production environments

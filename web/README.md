# AutoCoin Web Dashboard

Modern web dashboard for the AutoCoin Upbit automated trading system.

## Tech Stack

- **Framework**: Next.js 16 with App Router
- **React Version**: 19.0
- **Language**: TypeScript 5.7
- **Styling**: Tailwind CSS 3.4
- **Components**: Custom shadcn/ui-style components
- **Charts**: Recharts 2.15
- **Data Fetching**: SWR 2.3
- **Icons**: Lucide React

## Project Structure

```
web/
├── app/                      # Next.js App Router
│   ├── dashboard/           # Dashboard page
│   │   └── page.tsx         # Main dashboard component
│   ├── globals.css          # Global styles with CSS variables
│   ├── layout.tsx           # Root layout
│   └── page.tsx             # Home page
├── components/
│   ├── dashboard/           # Dashboard-specific components
│   │   ├── PortfolioSummary.tsx    # Portfolio overview card
│   │   ├── PositionCard.tsx         # Active position display
│   │   ├── AgentStatusGrid.tsx      # 6-agent status grid
│   │   ├── PnLChart.tsx             # 7-day PnL chart
│   │   ├── RecentTrades.tsx         # Recent trades table
│   │   └── QuickStats.tsx           # Quick statistics cards
│   └── ui/                   # Base UI components
│       ├── badge.tsx         # Badge component
│       ├── button.tsx        # Button component
│       ├── card.tsx          # Card components
│       ├── icons.tsx         # Icon exports
│       ├── separator.tsx     # Separator component
│       ├── skeleton.tsx      # Loading skeleton
│       └── table.tsx         # Table components
├── lib/
│   ├── api-client.ts         # API client with WebSocket
│   └── utils.ts             # Utility functions
├── types/
│   └── dashboard.ts         # TypeScript type definitions
└── package.json
```

## Getting Started

### Prerequisites

- Node.js 18+ or 20+
- npm or yarn
- AutoCoin Rust backend running on port 3000

### Installation

```bash
cd web
npm install
```

### Configuration

Create a `.env.local` file:

```bash
cp .env.example .env.local
```

Update the API URL if your backend runs on a different port:

```env
NEXT_PUBLIC_API_URL=http://localhost:3000
NEXT_PUBLIC_WS_URL=ws://localhost:3000/ws
```

### Development

```bash
npm run dev
```

Open [http://localhost:3001](http://localhost:3001) in your browser.

### Build

```bash
npm run build
npm start
```

## Components

### Dashboard Components

#### PortfolioSummary

Displays portfolio overview:
- Total asset value
- Available KRW balance
- Crypto asset value
- Asset allocation visualization

```tsx
<PortfolioSummary balance={balanceData} position={positionData} />
```

#### PositionCard

Shows active trading position:
- Market and current price
- Entry price
- Unrealized PnL (value and percentage)
- Stop loss and take profit levels
- Distance to exit points

```tsx
<PositionCard position={position} currentPrice={currentPrice} />
```

#### AgentStatusGrid

2x3 grid showing 6 agent statuses:
- Market Monitor
- Signal Detector
- Decision Maker
- Execution Agent
- Risk Manager
- Notification Agent

```tsx
<AgentStatusGrid agents={agentStates} />
```

#### PnLChart

7-day PnL history line chart:
- Daily PnL percentage
- Visual trend indicator
- Best/worst day stats
- Responsive design

```tsx
<PnLChart data={pnlHistoryData} />
```

#### RecentTrades

Table showing last 10 trades:
- Timestamp
- Market
- Side (Buy/Sell)
- Price
- Profit (for sells)

```tsx
<RecentTrades trades={tradesData} maxTrades={10} />
```

#### QuickStats

Four statistical cards:
- Win rate with W/L record
- Total trades count
- Today's PnL
- Total PnL

```tsx
<QuickStats stats={statsData} />
```

## API Integration

### REST API Endpoints

The dashboard expects the following endpoints from the Rust backend:

- `GET /api/dashboard` - Complete dashboard data
- `GET /api/balance` - Balance information
- `GET /api/position` - Current position
- `GET /api/agents` - Agent statuses
- `GET /api/trades?limit=10` - Recent trades
- `GET /api/pnl?days=7` - PnL history
- `GET /api/stats` - Trading statistics

### WebSocket

Real-time updates via WebSocket:

```typescript
// Message format
{
  type: "price_update" | "position_update" | "agent_status" | "trade_executed",
  data: unknown,
  timestamp: number
}
```

## Styling

### Theme

The dashboard uses CSS variables for theming:

```css
:root {
  --background: 0 0% 100%;
  --foreground: 240 10% 3.9%;
  --primary: 240 5.9% 10%;
  /* ... more variables */
}

.dark {
  --background: 240 10% 3.9%;
  --foreground: 0 0% 98%;
  /* ... more variables */
}
```

### Color Coding

- **Green**: Positive PnL, buy trades, running agents
- **Red**: Negative PnL, sell trades, errors
- **Yellow/Orange**: Warnings, pending status

## Performance

### Code Splitting

Components are automatically code-split by Next.js App Router.

### SWR Caching

Dashboard data is cached with 5-second refresh interval:
- Automatic revalidation on focus
- Reconnection handling
- Optimistic UI updates

### Responsive Design

- Mobile: Stacked layout
- Tablet: 2-column grid
- Desktop: Multi-column grid layout

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## License

MIT

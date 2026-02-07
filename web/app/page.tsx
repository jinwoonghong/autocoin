/**
 * Home Page
 *
 * Landing page with navigation to dashboard
 */

import Link from "next/link";

const FEATURES = [
  {
    title: "Real-Time Monitoring",
    description: "Track your trading agents and positions in real-time with live updates.",
  },
  {
    title: "Automated Trading",
    description: "Advanced momentum-based trading strategy with automatic entry and exit.",
  },
  {
    title: "Risk Management",
    description: "Built-in stop-loss and take-profit mechanisms to protect your investments.",
  },
  {
    title: "Performance Analytics",
    description: "Detailed PnL charts and statistics to track your trading performance.",
  },
];

export default function HomePage() {
  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b">
        <div className="container flex h-16 items-center justify-between px-4">
          <h1 className="text-xl font-bold">AutoCoin</h1>
          <nav className="flex items-center gap-4">
            <Link
              href="/dashboard"
              className="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
            >
              Dashboard
            </Link>
          </nav>
        </div>
      </header>

      {/* Hero Section */}
      <section className="container py-16 px-4 text-center">
        <div className="mx-auto max-w-3xl">
          <h2 className="text-4xl font-bold tracking-tight sm:text-5xl">
            Upbit Automated Trading Agent
          </h2>
          <p className="mt-4 text-lg text-muted-foreground">
            Advanced multi-agent trading system powered by Rust and Next.js.
            Monitor your portfolio, track performance, and manage risk in real-time.
          </p>
          <div className="mt-8 flex justify-center gap-4">
            <Link
              href="/dashboard"
              className="inline-flex items-center justify-center rounded-md bg-primary px-8 py-3 text-sm font-medium text-primary-foreground hover:bg-primary/90"
            >
              Go to Dashboard
            </Link>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="container py-12 px-4">
        <div className="mx-auto max-w-4xl">
          <h3 className="text-center text-2xl font-bold mb-8">Features</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
            {FEATURES.map((feature) => (
              <div
                key={feature.title}
                className="rounded-lg border bg-card p-6"
              >
                <h4 className="font-semibold mb-2">{feature.title}</h4>
                <p className="text-sm text-muted-foreground">
                  {feature.description}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t py-8">
        <div className="container px-4 text-center text-sm text-muted-foreground">
          <p>AutoCoin &copy; {new Date().getFullYear()}. Built with Rust and Next.js.</p>
        </div>
      </footer>
    </div>
  );
}

import type { Metadata } from "next";
import "./globals.css";
import { Navigation } from "@/components/layout/navigation";
import { ThemeProvider } from "@/components/providers/theme-provider";

export const metadata: Metadata = {
  title: "AutoCoin Dashboard",
  description: "Upbit Automated Trading Agent System Dashboard",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ko" suppressHydrationWarning>
      <body className="antialiased">
        <ThemeProvider
          attribute="class"
          defaultTheme="dark"
          enableSystem
          disableTransitionOnChange
        >
          <div className="flex min-h-screen">
            <Navigation />
            <main className="flex-1 lg:pl-64">
              {children}
            </main>
          </div>
        </ThemeProvider>
      </body>
    </html>
  );
}

/// <reference types="vite/client" />

interface Window {
  ThemeProvider: { updateWidget(theme?: string): void };
}

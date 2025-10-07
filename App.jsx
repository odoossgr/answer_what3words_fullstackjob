import React from 'react';
import { AppProvider } from '@shopify/polaris';
import { Provider as AppBridgeProvider } from '@shopify/app-bridge-react';
import SettingsPage from './pages/SettingsPage';

const config = {
  apiKey: import.meta.env.VITE_SHOPIFY_API_KEY,
  host: new URLSearchParams(window.location.search).get("host"),
  forceRedirect: true,
};

export default function App() {
  return (
    <AppBridgeProvider config={config}>
      <AppProvider>
        <SettingsPage />
      </AppProvider>
    </AppBridgeProvider>
  );
}

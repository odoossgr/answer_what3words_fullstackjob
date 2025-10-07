import React, { useState } from 'react';
import { Page, Card, Form, FormLayout, TextField, Button, Toast, Frame } from '@shopify/polaris';

export default function SettingsPage() {
  const [apiKey, setApiKey] = useState('');
  const [showToast, setShowToast] = useState(false);

  const handleSubmit = () => {
    // Giả lập lưu API key vào backend
    console.log("Saved API Key:", apiKey);
    setShowToast(true);
  };

  return (
    <Frame>
      <Page title="App Settings" subtitle="Configure your Shopify App">
        <Card sectioned>
          <Form onSubmit={handleSubmit}>
            <FormLayout>
              <TextField
                label="Your API Key"
                value={apiKey}
                onChange={setApiKey}
                helpText="Enter the API key you received from your service provider"
              />
              <Button submit primary>Save Settings</Button>
            </FormLayout>
          </Form>
        </Card>
        {showToast && (
          <Toast content="Settings saved!" onDismiss={() => setShowToast(false)} />
        )}
      </Page>
    </Frame>
  );
}

# 1. Create a new app (if not already created)
shopify app create node -n my-checkout-extension-app

cd my-checkout-extension-app

# 2. Scaffold a Checkout UI Extension
shopify app generate extension \
  --type=checkout_ui_extension \
  --name="My Checkout Extension" \
  --template=typescript-react

# 3. Start local dev + preview (opens browser)
shopify app dev

# 4. Build before deploying
shopify app build

# 5. Deploy to Shopify (for production or review)
shopify app deploy

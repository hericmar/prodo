# Prodo WebApp (prodo-webapp)

## Development
```bash
pnpm install
```

## Deployment
First of all, bump the version in `package.json` and `src-pwa/manifest.json`.
Then, run the following command:
```bash
pnpm build --mode pwa
```

Now, you can deploy the `dist/pwa` folder to your web server.

### Customize the configuration
See [Configuring quasar.config.js](https://v2.quasar.dev/quasar-cli-vite/quasar-config-js).

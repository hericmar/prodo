# Prodo WebApp (prodo-webapp)

## Development
```bash
pnpm install
```

## Deployment
First of all, create a git tag with bumped version.
Then, run the following command:
```bash
pnpm build --mode pwa
```

Now, you can deploy the `dist/pwa` folder to your web server.

### Customize the configuration
See [Configuring quasar.config.js](https://v2.quasar.dev/quasar-cli-vite/quasar-config-js).

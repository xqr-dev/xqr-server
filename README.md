# XQR (eXtended QR) code server

A simple http server that exposes a /.well-known/jwks.json endpoint for use with XQR codes.

## Features

- [x] Expose /.well-known/jwks.json endpoint for loaded keys
- [ ] Docker image
- [ ] Helm chart
- [ ] Support for multiple keys
- [ ] Cloudflare worker (store pubkey pem in env var or KV store?)

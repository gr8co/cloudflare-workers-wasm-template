# Cloudflare Workers WASM template

This project uses Wrangler 2.

- Open with VSCode using the .devcontainer for the fastest way to start programming
- run `npx wrangler dev` to start development locally (it will ask you to login to Cloudflare)
    - open `http://localhost:8787` once started

This project template showcases how you can call javascript functions within Rust through wasm-bindgen,
and how to use Cloudflare workers with WASM through webpack.

Use as a wrangler template:
```console
npx wrangler generate $NAME https://github.com/ronaldslc/cloudflare-workers-wasm-template
```

# Documentation

Official - https://developers.cloudflare.com/workers/platform/web-assembly/rust/

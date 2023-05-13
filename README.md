# Cloudflare Workers WASM template

This project uses Wrangler 2.

- Open with VSCode using the .devcontainer for the fastest way to start programming
- run `npx wrangler dev` to start development locally (it will ask you to login to Cloudflare)
    - open `http://localhost:8787` once started

This project template showcases how you can call javascript functions within Rust through wasm-bindgen,
and how to use Cloudflare Workers pure WASM

esbuild packages things that are included in package.json, if nodejs API is used you may need to add [extra flags](https://developers.cloudflare.com/workers/wrangler/configuration/#node-compatibility)

Use as a wrangler template:
```console
npm init cloudflare my-project https://github.com/gr8co/cloudflare-workers-wasm-template
# or
yarn create cloudflare my-project https://github.com/gr8co/cloudflare-workers-wasm-template
# or
$ pnpm create cloudflare my-project https://github.com/gr8co/cloudflare-workers-wasm-template
```

# Documentation

Official - https://developers.cloudflare.com/workers/platform/web-assembly/rust/

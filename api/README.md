# api

backend for personal portfolio hosted on a cloudflare worker.


## wrangler CLI usage

```bash
# Installing wrangler with `cargo` is deprecated.
npm install -g wrangler

# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

[Full Worker Documentation](https://docs.rs/worker)

## wasm

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as 
compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and
modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple. 

Read more about this on the [`workers-rs` project README](https://github.com/cloudflare/workers-rs).

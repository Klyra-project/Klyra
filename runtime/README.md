# Klyra - Deploy Rust apps with a single command

<div style="display: flex; margin-top: 30px; margin-bottom: 30px;">
<img src="https://raw.githubusercontent.com/klyra-hq/klyra/main/assets/logo-rectangle-transparent.png" width="400px" style="margin-left: auto; margin-right: auto;"/>
</div>

[Klyra](https://www.klyra.dev/) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

📖 Check out our documentation to get started quickly: [docs.klyra.dev](https://docs.klyra.dev)

🙋‍♂️ If you have any questions, join our [Discord](https://discord.gg/klyra) server.

## Usage

Start by installing the Klyra CLI by running the following in a terminal:

```bash
cargo install cargo-klyra
```

Now that Klyra is installed, you can initialize a project with Axum boilerplate:

```bash
klyra init --template axum my-axum-app
```

By looking at the `Cargo.toml` file of the generated `my-axum-app` project you will see it has been made to
be a binary crate with a few dependencies including `klyra-runtime` and `klyra-axum`.

```toml
axum = "0.8.1"
klyra-axum = "0.54.0"
klyra-runtime = "0.54.0"
tokio = "1.28.2"
```

A boilerplate code for your axum project can also be found in `src/main.rs`:

```rust,no_run
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn main() -> klyra_axum::KlyraAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

Check out [our docs](https://docs.klyra.dev) to see all the frameworks we support, or
our [examples](https://github.com/klyra-hq/klyra-examples) if you prefer that format.

## Running locally

To test your app locally before deploying, use:

```bash
klyra run
```

You should see your app build and start on the default port 8000. You can test this using;

```bash
curl http://localhost:8000/
# Hello, world!
```

## Deploying

Deploy the service with:

```bash
klyra deploy
```

Your service will then be made available under a subdomain of `*.klyra.app`. For example:

```bash
curl https://my-axum-app-0000.klyra.app/
# Hello, world!
```

## Klyra service integration for the Axum web framework.

Axum 0.7 is now supported by using these feature flags:
```toml,ignore
axum = "0.7.0"
klyra-axum = { version = "0.35.0", default-features = false, features = ["axum-0-7"] }
```

### Example

```rust,ignore
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn axum() -> klyra_axum::KlyraAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

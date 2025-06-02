## Klyra service integration for the Axum web framework.

### Example

```rust,no_run
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

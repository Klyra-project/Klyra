## Klyra service integration for the Actix Web framework.

### Example

```rust,no_run
use actix_web::{get, web::ServiceConfig};
use klyra_actix_web::KlyraActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[klyra_runtime::main]
async fn actix_web() -> KlyraActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}

```

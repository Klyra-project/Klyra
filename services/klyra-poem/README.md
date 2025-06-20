## Klyra service integration for the Poem web framework

### Example

```rust,no_run
use poem::{get, handler, Route};
use klyra_poem::KlyraPoem;

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn poem() -> KlyraPoem<impl poem::Endpoint> {
    let app = Route::new().at("/", get(hello_world));

    Ok(app.into())
}
```

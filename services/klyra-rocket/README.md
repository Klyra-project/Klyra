## Klyra service integration for the Rocket web framework

### Example

```rust,no_run
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn rocket() -> klyra_rocket::KlyraRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
```

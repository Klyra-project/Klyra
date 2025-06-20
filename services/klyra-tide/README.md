## Klyra service integration for the Tide web framework

### Example

```rust,no_run
#[klyra_runtime::main]
async fn tide() -> klyra_tide::KlyraTide<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });

    Ok(app.into())
}
```

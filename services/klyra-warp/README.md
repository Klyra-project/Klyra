## Klyra service integration for the Warp web framework.

### Example

```rust,no_run
use warp::Filter;
use warp::Reply;

#[klyra_runtime::main]
async fn warp() -> klyra_warp::KlyraWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed().into())
}
```

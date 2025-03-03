use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_service::main]
async fn axum() -> klyra_service::KlyraAxum {
    let router = Router::new().route("/hello", get(hello_world));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

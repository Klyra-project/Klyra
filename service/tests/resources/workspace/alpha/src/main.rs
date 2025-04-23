use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    shared::hello()
}

#[klyra_runtime::main]
async fn axum() -> klyra_axum::KlyraAxum {
    let router = Router::new().route("/hello", get(hello_world));

    Ok(router.into())
}

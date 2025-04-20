#[klyra_runtime::main]
async fn axum() -> klyra_axum::KlyraAxum {
    let router = axum::Router::new();

    Ok(router.into())
}

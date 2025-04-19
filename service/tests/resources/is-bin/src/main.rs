#[klyra_service::main]
async fn axum() -> klyra_service::KlyraAxum {
    let router = axum::Router::new();

    Ok(router)
}

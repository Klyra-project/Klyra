// This service cannot be hosted on klyra since it is missing the runtime the klyra main macro would have added!!!
async fn axum() -> klyra_axum::KlyraAxum {
    let router = axum::Router::new();

    Ok(router.into())
}

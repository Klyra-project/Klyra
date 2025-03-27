use warp::Filter;
use warp::Reply;

#[klyra_service::main]
async fn warp() -> klyra_service::KlyraWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed())
}

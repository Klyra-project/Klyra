use poem::{get, handler, Route};

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_service::main]
async fn main() -> klyra_service::KlyraPoem<impl poem::Endpoint> {
    let app = Route::new().at("/hello", get(hello_world));

    Ok(app)
}

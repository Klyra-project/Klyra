use klyra_service::Service;

struct MyService;

#[klyra_service::async_trait]
impl Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), klyra_service::Error> {
        panic!("panic in bind");
    }
}

#[klyra_service::main]
async fn bind_panic() -> Result<MyService, klyra_service::Error> {
    Ok(MyService)
}

use klyra_service::Service;

struct MyService;

#[klyra_service::async_trait]
impl Service for MyService {
    async fn bind(
        mut self: Box<Self>,
        _: std::net::SocketAddr,
    ) -> Result<(), klyra_service::Error> {
        Ok(())
    }
}

#[klyra_service::main]
async fn main_panic() -> Result<MyService, klyra_service::Error> {
    panic!("main panic")
}

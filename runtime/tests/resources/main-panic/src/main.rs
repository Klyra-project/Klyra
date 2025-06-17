struct MyService;

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), klyra_runtime::Error> {
        Ok(())
    }
}

#[klyra_runtime::main]
async fn panic_message() -> Result<MyService, klyra_runtime::Error> {
    panic!("panic in main");
}

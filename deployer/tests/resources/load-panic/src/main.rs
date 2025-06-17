struct MyService;

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for MyService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), klyra_runtime::Error> {
        Ok(())
    }
}

#[derive(Default)]
struct Thing;

#[klyra_runtime::async_trait]
impl klyra_service::ResourceInputBuilder for Thing {
    type Input = ();
    type Output = ();

    async fn build(
        self,
        _factory: &klyra_service::ResourceFactory,
    ) -> Result<Self::Input, klyra_service::Error> {
        panic!("load panic");
    }
}

#[klyra_runtime::main]
async fn load_panic(#[Thing] _a: ()) -> Result<MyService, klyra_runtime::Error> {
    Ok(MyService)
}

klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = pet, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[klyra_codegen::endpoint(method =, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }
}

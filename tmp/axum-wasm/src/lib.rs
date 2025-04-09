klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[klyra_codegen::endpoint(method = get, route = "/goodbye")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

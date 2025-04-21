klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = get, method = get)]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[klyra_codegen::endpoint(route = "/goodbye", route = "/goodbye")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

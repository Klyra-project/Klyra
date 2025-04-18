klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }

    #[klyra_codegen::endpoint(method = post, route = "/hello")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }

    #[klyra_codegen::endpoint(method = post, route = "/hello")]
    async fn goodbye() -> &'static str {
        "Goodbye, World!"
    }
}

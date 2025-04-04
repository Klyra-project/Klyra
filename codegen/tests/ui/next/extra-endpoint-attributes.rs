klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = get, route = "/hello")]
    #[klyra_codegen::endpoint(method = post, route = "/hello")]
    async fn hello() -> &'static str {
        "Hello, World!"
    }
}

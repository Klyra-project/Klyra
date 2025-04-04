klyra_codegen::app! {
    #[klyra_codegen::endpoint(method = get)]
    async fn only_method_param() -> &'static str {
        "Hello, World!"
    }

    #[klyra_codegen::endpoint(route = "/goodbye")]
    async fn only_route_param() -> &'static str {
        "Goodbye, World!"
    }

    #[klyra_codegen::endpoint()]
    async fn no_params() -> &'static str {
        "Goodbye, World!"
    }
}

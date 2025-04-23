klyra_next::app! {
    #[klyra_next::endpoint(method = get, route = "/hello")]
    async fn hello() -> &'static str {
        shared::hello()
    }
}

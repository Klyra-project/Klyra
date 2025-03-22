use salvo::prelude::*;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello, world!"));
}

#[klyra_service::main]
async fn salvo() -> klyra_service::KlyraSalvo {
    let router = Router::with_path("hello").get(hello_world);

    Ok(router)
}

#[cfg(feature = "next")]
mod next;
#[cfg(feature = "frameworks")]
mod klyra_main;

/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// ## Without klyra managed resources
/// The simplest usage is when your service does not require any klyra managed resources, so you only need to return a klyra supported service:
///
/// ```rust,no_run
/// use klyra_rocket::KlyraRocket;
///
/// #[klyra_rocket::main]
/// async fn rocket() -> KlyraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket.into())
/// }
/// ```
///
/// ## Klyra supported services
/// The following types can be returned from a `#[klyra_service::main]` function and enjoy first class service support in klyra.
///
/// | Return type                           | Crate                                                         | Service                                     | Version    | Example                                                                               |
/// | ------------------------------------- |-------------------------------------------------------------- | ------------------------------------------- | ---------- | -----------------------------------------------------------------------------------   |
/// | `KlyraActixWeb`                     |[klyra-actix-web](https://crates.io/crates/klyra-actix-web)| [actix-web](https://docs.rs/actix-web/4.3)  | 4.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/actix-web/hello-world)      |
/// | `KlyraAxum`                         |[klyra-axum](https://crates.io/crates/klyra-axum)          | [axum](https://docs.rs/axum/0.6)            | 0.5        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/axum/hello-world)           |
/// | `KlyraPoem`                         |[klyra-poem](https://crates.io/crates/klyra-poem)          | [poem](https://docs.rs/poem/1.3)            | 1.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poem/hello-world)           |
/// | `KlyraPoise`                        |[klyra-poise](https://crates.io/crates/klyra-poise)        | [poise](https://docs.rs/poise/0.5)          | 0.5        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poise/hello-world)          |
/// | `KlyraRocket`                       |[klyra-rocket](https://crates.io/crates/klyra-rocket)      | [rocket](https://docs.rs/rocket/0.5.0-rc.2) | 0.5.0-rc.2 | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/rocket/hello-world)         |
/// | `KlyraSalvo`                        |[klyra-salvo](https://crates.io/crates/klyra-salvo)        | [salvo](https://docs.rs/salvo/0.63)         | 0.63       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/salvo/hello-world)          |
/// | `KlyraSerenity`                     |[klyra-serenity](https://crates.io/crates/klyra-serenity   | [serenity](https://docs.rs/serenity/0.11)   | 0.11       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/serenity/hello-world)       |
/// | `KlyraThruster`                     |[klyra-thruster](https://crates.io/crates/klyra-thruster)  | [thruster](https://docs.rs/thruster/1.3)    | 1.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/thruster/hello-world)       |
/// | `KlyraTower`                        |[klyra-tower](https://crates.io/crates/klyra-tower)        | [tower](https://docs.rs/tower/0.4)          | 0.4        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tower/hello-world)          |
/// | `KlyraTide`                         |[klyra-tide](https://crates.io/crates/klyra-tide)          | [tide](https://docs.rs/tide/0.16)           | 0.16       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tide/hello-world)           |
///
/// ## Getting klyra managed resources
/// Klyra is able to manage resource dependencies for you. These resources are passed in as inputs to your `#[klyra_runtime::main]` function and are configured using attributes:
/// ```rust,no_run
/// use sqlx::PgPool;
/// use klyra_rocket::KlyraRocket;
///
/// struct MyState(PgPool);
///
/// #[klyra_runtime::main]
/// async fn rocket(#[klyra_shared_db::Postgres] pool: PgPool) -> KlyraRocket {
///     let state = MyState(pool);
///     let rocket = rocket::build().manage(state);
///
///     Ok(rocket.into())
/// }
/// ```
///
/// More [klyra managed resources can be found here](https://github.com/klyra-hq/klyra/tree/main/resources)
#[cfg(feature = "frameworks")]
#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    klyra_main::r#impl(attr, item)
}

/// Generates a wasm32-wasi module containing an Axum router with your endpoints, which is passed as a
/// hyper::service::Service to a hyper::Server.
///
/// ## Example
///
/// ```
/// klyra_next::app! {
///     use futures::TryStreamExt;
///     use tracing::debug;
///     use klyra_next::body::StreamBody;
///     use klyra_next::extract::BodyStream;
///     use klyra_next::response::{Response, IntoResponse};
///
///     #[klyra_next::endpoint(method = get, route = "/")]
///     async fn hello() -> &'static str {
///         "Hello, World!"
///     }
///
///     // We can also use tracing/log macros directly:
///     #[klyra_next::endpoint(method = get, route = "/goodbye")]
///     async fn goodbye() -> &'static str {
///         debug!("goodbye endpoint called");
///         "Goodbye, World!"
///     }
///
///     // We can also extract the http body in our handlers.
///     // The endpoint below takes the body from the request using the axum `BodyStream`
///     // extractor, lazily maps its bytes to uppercase and streams it back in our response:
///     #[klyra_next::endpoint(method = post, route = "/uppercase")]
///     async fn uppercase(body: BodyStream) -> impl IntoResponse {
///         let chunk_stream = body.map_ok(|chunk| {
///             chunk
///                 .iter()
///                 .map(|byte| byte.to_ascii_uppercase())
///                 .collect::<Vec<u8>>()
///         });
///         Response::new(StreamBody::new(chunk_stream))
///     }
/// }
/// ```
#[cfg(feature = "next")]
#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn app(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use next::App;
    use syn::{parse_macro_input, File};

    let mut file = parse_macro_input!(item as File);

    let app = App::from_file(&mut file);
    let bindings = next::wasi_bindings(app);

    quote::quote!(
        #file
        #bindings
    )
    .into()
}

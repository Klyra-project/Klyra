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
/// | Return type       | Crate                                                          | Service                                                                          | Version    | Example                                                                                 |
/// | ----------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------------- | ---------- | --------------------------------------------------------------------------------------- |
/// | `KlyraActixWeb` | [klyra-actix-web](https://crates.io/crates/klyra-actix-web)| [actix-web](https://docs.rs/actix-web/4.3)                                       | 4.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/actix-web/hello-world)|
/// | `KlyraAxum`     | [klyra-axum](https://crates.io/crates/klyra-axum)          | [axum](https://docs.rs/axum/0.7)                                                 | 0.7        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/axum/hello-world)     |
/// | `KlyraPoem`     | [klyra-poem](https://crates.io/crates/klyra-poem)          | [poem](https://docs.rs/poem/2.0)                                                 | 2.0        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poem/hello-world)     |
/// | `KlyraRocket`   | [klyra-rocket](https://crates.io/crates/klyra-rocket)      | [rocket](https://docs.rs/rocket/0.5)                                             | 0.5        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/rocket/hello-world)   |
/// | `KlyraSalvo`    | [klyra-salvo](https://crates.io/crates/klyra-salvo)        | [salvo](https://docs.rs/salvo/0.63)                                              | 0.63       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/salvo/hello-world)    |
/// | `KlyraSerenity` | [klyra-serenity](https://crates.io/crates/klyra-serenity)  | [serenity](https://docs.rs/serenity/0.12) and [poise](https://docs.rs/poise/0.6) | 0.12       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/serenity/hello-world) |
/// | `KlyraThruster` | [klyra-thruster](https://crates.io/crates/klyra-thruster)  | [thruster](https://docs.rs/thruster/1.3)                                         | 1.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/thruster/hello-world) |
/// | `KlyraTower`    | [klyra-tower](https://crates.io/crates/klyra-tower)        | [tower](https://docs.rs/tower/0.4)                                               | 0.4        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tower/hello-world)    |
/// | `KlyraTide`     | [klyra-tide](https://crates.io/crates/klyra-tide)          | [tide](https://docs.rs/tide/0.16)                                                | 0.16       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tide/hello-world)     |
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
#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    klyra_main::tokens(attr, item)
}

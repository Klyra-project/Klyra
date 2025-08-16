mod klyra_main;

/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// ## Without klyra managed resources
/// The simplest usage is when your service does not require any klyra managed resources, so you only need to return a klyra supported service:
///
/// ```rust,no_run
/// use klyra_rocket::KlyraRocket;
///
/// #[klyra_runtime::main]
/// async fn rocket() -> KlyraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket.into())
/// }
/// ```
///
/// ## Klyra supported services
/// The following types can be returned from a `#[klyra_runtime::main]` function and enjoy first class service support in klyra.
///
/// | Return type       | Crate                                                          | Service                                                                 | Example                                                                                 |
/// | ----------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
/// | `KlyraActixWeb` | [klyra-actix-web](https://crates.io/crates/klyra-actix-web)| [actix-web](https://docs.rs/actix-web)                                  | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/actix-web/hello-world)|
/// | `KlyraAxum`     | [klyra-axum](https://crates.io/crates/klyra-axum)          | [axum](https://docs.rs/axum)                                            | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/axum/hello-world)     |
/// | `KlyraPoem`     | [klyra-poem](https://crates.io/crates/klyra-poem)          | [poem](https://docs.rs/poem)                                            | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poem/hello-world)     |
/// | `KlyraRocket`   | [klyra-rocket](https://crates.io/crates/klyra-rocket)      | [rocket](https://docs.rs/rocket)                                        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/rocket/hello-world)   |
/// | `KlyraSalvo`    | [klyra-salvo](https://crates.io/crates/klyra-salvo)        | [salvo](https://docs.rs/salvo)                                          | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/salvo/hello-world)    |
/// | `KlyraSerenity` | [klyra-serenity](https://crates.io/crates/klyra-serenity)  | [serenity](https://docs.rs/serenity) and [poise](https://docs.rs/poise) | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/serenity/hello-world) |
/// | `KlyraThruster` | [klyra-thruster](https://crates.io/crates/klyra-thruster)  | [thruster](https://docs.rs/thruster)                                    | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/thruster/hello-world) |
/// | `KlyraTower`    | [klyra-tower](https://crates.io/crates/klyra-tower)        | [tower](https://docs.rs/tower)                                          | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tower/hello-world)    |
/// | `KlyraTide`     | [klyra-tide](https://crates.io/crates/klyra-tide)          | [tide](https://docs.rs/tide)                                            | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tide/hello-world)     |
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
#[proc_macro_error2::proc_macro_error]
#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    klyra_main::tokens(attr, item)
}

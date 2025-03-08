# Overview
This project is meant to run all the end-to-end tests for klyra. Here are some notes to help you in your testing
journey.

## Making changes to klyra-service
The examples pull `klyra-service` from crates.io. Therefore, any changes made to `klyra-service` will not be detected
until they are published to crates.io. A way around this is to use the
[`[patch]`](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section) section in
`Cargo.toml` to use the changed `klyra-service` instead. Create a `.cargo/config.toml` in your
[config folder](https://doc.rust-lang.org/cargo/reference/config.html) with the following content.

``` toml
[patch.crates-io]
klyra-service = { path = "[base]/klyra/service" }
```

Now the tests will run against the changes made in `klyra-service`.

<p align="center">
<img width="300" src="https://raw.githubusercontent.com/getsynth/klyra/master/resources/logo-rectangle-transparent.png"/>
</p>
<br>
<p align=center>
  <a href="https://docs.rs/klyra-service">
    <img alt="docs" src="https://img.shields.io/badge/doc-reference-orange">
  </a>
  <a href="https://github.com/getsynth/klyra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://github.com/getsynth/klyra/actions">
    <img alt="build status" src="https://img.shields.io/github/workflow/status/getsynth/klyra/cargo-test"/>
  </a>
  <a href="https://discord.gg/H33rRDTm3p">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
</p>

---

# klyra

[Klyra](https://www.klyra.rs/) is a serverless platform for Rust which makes it really easy to 
deploy your web-apps.

Klyra is built for productivity, reliability and performance:
- Zero-Configuration support for Rust using annotations
- Automatic resource provisioning (databases, caches, subdomains, etc.) via [Infrastructure-From-Code](https://www.klyra.rs/blog/2022/05/09/ifc)
- First-class support for popular Rust frameworks ([Rocket](https://github.com/klyra-hq/klyra/tree/main/examples/rocket/hello-world), [Axum](https://github.com/klyra-hq/klyra/tree/main/examples/axum/hello-world), 
  [Tide](https://github.com/klyra-hq/klyra/tree/main/examples/tide/hello-world) and [Tower](https://github.com/klyra-hq/klyra/tree/main/examples/tower/hello-world))
- Scalable hosting (with optional self-hosting)


## Getting Started

First download the Klyra cargo extension and login:

```bash
$ cargo install cargo-klyra
$ cargo klyra login
```

Create your first klyra app with `rocket` framework:

```bash
$ cargo klyra init --rocket hello-world
```

Your `Cargo.toml` should look like:

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[lib]

[dependencies]
klyra-service = { version = "0.4.0", features = ["web-rocket"] }
rocket = "0.4.11"
```


Your klyra app in `lib.rs` should look like:

```rust
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_service::main]
async fn rocket() -> klyra_service::KlyraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket)
}
```

Deploy:

```bash
$ cargo klyra deploy
   Finished dev [unoptimized + debuginfo] target(s) in 1m 01s

        Project:            hello-world
        Deployment Id:      3d08ac34-ad63-41c1-836b-99afdc90af9f
        Deployment Status:  DEPLOYED
        Host:               hello-world.klyraapp.rs
        Created At:         2022-04-01 08:32:34.412602556 UTC
        Database URI:       postgres://***:***@pg.klyra.rs/db-hello-world
```

For the full documentation, visit [docs.rs/klyra-service](https://docs.rs/klyra-service)

## Contributing to klyra

If you want to setup a local environment to test code changes to core `klyra` packages, or want to contribute to the project see [CONTRIBUTING.md](https://github.com/klyra-hq/klyra/blob/main/CONTRIBUTING.md)

## Roadmap

For a comprehensive view of the klyra roadmap check out this [project board](https://github.com/orgs/klyra-hq/projects/4).

If you have any requests or suggestions feel free to open an issue.

## Community & Support

- [Community Forum](https://github.com/getsynth/klyra/discussions). Best for: help with building, discussion about best practices.
- [GitHub Issues](https://github.com/getsynth/klyra/issues). Best for: bugs and errors you encounter using Klyra.
- [Discord](https://discord.gg/H33rRDTm3p). Best for: sharing your applications and hanging out with the community.
- [Twitter](https://twitter.com/klyra_dev). Best for: keeping up with announcements and releases

## Status

- [x] Alpha: We are testing Klyra, API and deployments may be unstable
- [x] Public Alpha: Anyone can sign up, but go easy on us, 
  there are a few kinks
- [ ] Public Beta: Stable enough for most non-enterprise use-cases
- [ ] Public: Production-ready!

We are currently in Public Alpha. Watch "releases" of this repo to get 
notified of major updates!

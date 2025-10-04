<!-- markdownlint-disable -->
<p align="center">
<img width="200" src="https://github.com/user-attachments/assets/c9190502-9423-4c6e-8fb5-5aecce4cd85e"/>
</p>

<h1 align="center">Swiftly Create & Launch Rust Applications</h1>
<div align="center">
Provision resources and deploy your projects with minimal code.
</div>

<h3 align="center">Effortless. Streamlined. Delightful.</h3>


## Key Features

- **Single-Line Resource Setup:** Add a database or other resources with just one line in your main file—no need for configuration or YAML files.
- **Fast Development Cycle:** Go from project setup to deployment in just 2 minutes. Provision resources in seconds and push to production effortlessly.
- **Top Rust Frameworks Supported:** Full compatibility with [Axum](https://docs.klyra.dev/examples/axum), [Actix Web](https://docs.klyra.dev/examples/actix), [Rocket](https://docs.klyra.dev/examples/rocket), and [more](https://docs.klyra.dev/examples/other).
- **Secure by Default:** Focus on coding while we handle security and permissions for you.
<br>
<br>

## Getting Started

For Linux or macOS, run this installation script to automatically set up the appropriate version for your system:

```sh
curl -sSfL https://www.klyra.dev/install | bash
```

For Windows, use the following script to achieve the same:

```powershell
iwr "https://www.klyra.dev/install-win" | iex
```

Once installed, log in with:

```sh
klyra login
```

To start a new project, run:

```bash
klyra init --template axum hello-world
```

To deploy it, navigate to the project folder and execute:

```bash
cd hello-world
klyra deploy
```

That’s all it takes!

```text
Service Name:  hello-world
Deployment ID: 3d08ac34-ad63-41c1-836b-99afdc90af9f
Status:        active
Last Updated:  2022-04-01T08:32:34Z
URI:           https://hello-world.klyraapp.rs
```

You can build upon the `hello-world` template or explore our [example projects](https://github.com/klyra-project/klyra-examples).

For comprehensive guides, check out [our documentation](https://docs.klyra.dev).
<br>
<br>

## Example Overview

Here’s a simple "Hello World" app using Axum:

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
```

To make it deployable with a single command, modify it like this:

```rust
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn main() -> klyra_axum::KlyraAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

Now, running `klyra deploy` will make your app live. To add a shared Postgres database, update the code as follows:

```rust
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn main(
    #[klyra_shared_db::Postgres] pool: sqlx::PgPool,
) -> klyra_axum::KlyraAxum {

    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("failed to run migrations");

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
```

Run `klyra deploy` again, and your project will be live with a fully configured database.
<br>
<br>

## Repositories

| Name | Description |
|-|-|
| [klyra](https://github.com/klyra-project/klyra) 🚀 (This repo) | Contains all library crates and the Klyra CLI tool. |
| [klyra-examples](https://github.com/klyra-project/klyra-examples) 👨‍🏫 | Official example projects deployable on Klyra. |
| [klyra-docs](https://docs.klyra.dev/) 📃 | Official documentation at [docs.klyra.dev](https://docs.klyra.dev/). |
<br>

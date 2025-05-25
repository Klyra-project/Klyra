<!-- markdownlint-disable -->
<div align="center">

# cargo-klyra

<p align=center>
  <a href="https://docs.rs/klyra-service">
    <img alt="docs" src="https://img.shields.io/badge/docs-reference-orange">
  </a>
  <a href="https://github.com/klyra-hq/klyra/search?l=rust">
    <img alt="language" src="https://img.shields.io/badge/language-Rust-orange.svg">
  </a>
  <a href="https://circleci.com/gh/klyra-hq/klyra/">
    <img alt="build status" src="https://circleci.com/gh/klyra-hq/klyra.svg?style=shield"/>
  </a>
  <a href="https://discord.gg/klyra">
    <img alt="discord" src="https://img.shields.io/discord/803236282088161321?logo=discord"/>
  </a>
</p>
<!-- markdownlint-restore -->
<!-- markdownlint-disable MD001 -->

`cargo-klyra` is your commandline tool for deploying web apps on [klyra](https://www.klyra.rs/), the stateful serverless web platform for Rust.

**README Sections:** [Installation](#installation) — [Subcommands](#subcommands) — [Development](#development)

</div>

---

`cargo-klyra` brings [klyra](https://www.klyra.rs/), the open source serverless platform for Rust web applications, into your terminal. With a dedicated focus on productivity, reliability, and performance, `cargo-klyra` makes deploying your code to the cloud as easy as deriving a trait.

---

<!-- markdownlint-disable-next-line -->
<a id="installation"><h1>Installation</h1></a>

`cargo-klyra` is available for macOS, Linux, and Windows.

To install on Linux or macOS, run:

```sh
curl -sSfL https://www.klyra.rs/install | bash
```

On Windows, you can default to installing from source:

```bash
cargo install cargo-klyra
```

### Distro Packages

<!-- markdownlint-disable-next-line -->
<details>
<!-- markdownlint-disable-next-line -->
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/cargo-klyra.svg)](https://repology.org/project/cargo-klyra/versions)

</details>

#### Arch Linux

`cargo-klyra` can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/cargo-klyra) using [pacman](https://wiki.archlinux.org/title/Pacman):

```sh
pacman -S cargo-klyra
```

---

<!-- markdownlint-disable-next-line -->
<a id="subcommands"><h1>Subcommands</h1></a>

`cargo-klyra`'s subcommands help you build and deploy web apps from start to finish.

Run `cargo klyra help` to see the basic usage:

```text
Usage: cargo-klyra [OPTIONS] <COMMAND>

Commands:
  init        Create a new klyra project
  run         Run a klyra service locally
  deploy      Deploy a klyra service
  deployment  Manage deployments of a klyra service
  status      View the status of a klyra service
  stop        Stop this klyra service
  logs        View the logs of a deployment in this klyra service
  project     List or manage projects on klyra
  resource    Manage resources of a klyra project
  secrets     Manage secrets for this klyra service
  clean       Remove cargo build artifacts in the klyra environment
  login       Login to the klyra platform
  logout      Log out of the klyra platform
  generate    Generate shell completions
  feedback    Open an issue on GitHub and provide feedback
  help        Print this message or the help of the given subcommand(s)

Options:
      --working-directory <WORKING_DIRECTORY>  Specify the working directory [default: .]
      --name <NAME>                            Specify the name of the project (overrides crate name)
      --api-url <API_URL>                      Run this command against the API at the supplied URL (allows targeting a custom deployed instance for this command only, mainly
                                               for development) [env: klyra_API=]
  -h, --help                                   Print help
  -V, --version                                Print version
```

### Subcommand: `init`

To initialize a klyra project with boilerplates, run `cargo klyra init [OPTIONS] [PATH]`.

Currently, `cargo klyra init` supports the following frameworks:

- `--template actix-web`: for [actix web](https://actix.rs/) framework
- `--template axum`: for [axum](https://github.com/tokio-rs/axum) framework
- `--template poem`: for [poem](https://github.com/poem-web/poem) framework
- `--template poise`: for [poise](https://github.com/serenity-rs/poise) discord bot framework
- `--template rocket`: for [rocket](https://rocket.rs/) framework
- `--template salvo`: for [salvo](https://salvo.rs/) framework
- `--template serenity`: for [serenity](https://github.com/serenity-rs/serenity) discord bot framework
- `--template thruster`: for [thruster](https://github.com/thruster-rs/Thruster) framework
- `--template tide`: for [tide](https://github.com/http-rs/tide) framework
- `--template tower`: for [tower](https://github.com/tower-rs/tower) library
- `--template warp`: for [warp](https://github.com/seanmonstar/warp) framework

For example, running the following command will initialize a project for [rocket](https://rocket.rs/):

```sh
cargo klyra init --template rocket my-rocket-app
```

This should generate the following dependency in `Cargo.toml`:

```toml
rocket = "0.5.0-rc.2"
klyra-rocket = { version = "0.29.0" }
klyra-runtime = { version = "0.29.0" }
tokio = { version = "1.26.0" }
```

The following boilerplate code should be generated into `src/lib.rs`:

```rust
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn rocket() -> klyra_rocket::KlyraRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
```

### Subcommand: `run`

To run the klyra project locally, use the following command:

```sh
# Inside your klyra project
cargo klyra run
```

This will compile your klyra project and start it on the default port `8000`. Test it by:

```sh
$ curl http://localhost:8000
Hello, world!
```

### Subcommand: `login`

Use `cargo klyra login` inside your klyra project to generate an API key for the klyra platform:

```sh
# Inside a klyra project
cargo klyra login
```

This should automatically open a browser window with an auto-generated API key for your project. Simply copy-paste the API key back in your terminal or run the following command to complete login:

```sh
cargo klyra login --api-key <your-api-key-from-browser>
```

### Subcommand: `deploy`

To deploy your klyra project to the cloud, run:

```sh
cargo klyra project start
cargo klyra deploy
```

Your service will immediately be available at `{crate_name}.klyraapp.rs`. For instance:

```sh
$ curl https://my-rocket-app.klyraapp.rs
Hello, world!
```

### Subcommand: `status`

Check the status of your deployed klyra project with:

```sh
cargo klyra status
```

### Subcommand: `logs`

Check the logs of your deployed klyra project with:

```sh
cargo klyra logs
```

### Subcommand: `stop`

Once you are done with a deployment, you can stop it by running:

```sh
cargo klyra stop
```

---

<!-- markdownlint-disable-next-line -->
<a id="development"><h1>Development</h1></a>

Thanks for using `cargo-klyra`! We’re very happy to have you with us!

To contribute to `cargo-klyra` or stay updated with our development, please [open an issue, discussion or PR on Github](https://github.com/klyra-hq/klyra) and [join our Discord](https://discord.gg/klyra)! 🚀

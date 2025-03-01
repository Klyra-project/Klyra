# Url Shortener

A URL shortener that you can use from your terminal - built with klyra, rocket and postgres/sqlx.

## How to use it

You can use this URL shortener directly from your terminal. Just copy and paste this command to your terminal and replace `<URL>` with the URL that you want to shorten

```bash
curl -X POST -d '<URL>' https://s.klyraapp.rs
```

like this

```bash
curl -X POST -d 'https://docs.rs/klyra-service/latest/klyra_service/' https://s.klyraapp.rs
```

you will get the shortened URL back (something like this `https://s.klyraapp.rs/RvpVU_`)

## Project structure

The project consists of the following files

- `Klyra.toml` contains the name of the app (if name is `s` domain will be `s.klyraapp.rs`)
- `migrations` folder is for DB migration files created by [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)
- `src/lib.rs` is where all the magic happens - it creates a klyra service with two endpoints: one for creating new short URLs and one for handling shortened URLs.

## How to deploy

To deploy this app, check out the repository locally

```bash
$ git clone https://github.com/getsynth/klyra.git
```

navigate to `examples/url-shortener`

```bash
$ cd examples/url-shortener
```

install klyra

```bash
$ cargo install cargo-klyra
```

login to klyra

```bash
$ cargo klyra login
```

Pick a project name that is something unique - in klyra,
projects are globally unique. Then run

```bash
$ cargo klyra deploy --name=$PROJECT_NAME
```

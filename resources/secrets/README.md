# Klyra Secrets
This plugin manages secrets on [klyra](https://www.klyra.rs).

## Usage
Add `klyra-secrets` to the dependencies for your service. Also add a dependency which will give you a `PgPool` like [klyra-shared-db](https://github.com/klyra-hq/klyra/tree/main/resources/shared-db)

[`SecretStore::get_secret`] can now be called on any instance of this pool to retrieve stored secrets.

An example using the Rocket framework can be found on [GitHub](https://github.com/klyra-hq/klyra/tree/main/examples/rocket/postgres)


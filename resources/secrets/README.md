# Klyra Secrets
This plugin manages secrets on [klyra](https://www.klyra.rs).

## Usage
Add `klyra-secrets` to the dependencies for your service, and add a `Secrets.toml` to the root of your project
with the secrets you'd like to store. Make sure to add `Secrets.toml` to a `.gitignore` to omit your secrets from version control.

Next, pass `#[klyra_secrets::Secrets] secret_store: SecretStore` as an argument to your `klyra_service::main` function.
`SecretStore::get` can now be called to retrieve your API keys and other secrets at runtime.

An example using the Rocket framework can be found on [GitHub](https://github.com/klyra-hq/klyra/tree/main/examples/rocket/secrets)

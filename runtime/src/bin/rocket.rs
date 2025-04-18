// The few line below is what we should now codegen for legacy
#[tokio::main]
async fn main() {
    klyra_runtime::start(loader).await;
}

async fn loader<S: klyra_common::storage_manager::StorageManager>(
    mut factory: klyra_runtime::ProvisionerFactory<S>,
) -> klyra_service::KlyraRocket {
    use klyra_service::ResourceBuilder;

    let secrets = klyra_secrets::Secrets::new().build(&mut factory).await?;

    rocket(secrets).await
}

// Everything below this is the usual code a user will write
use anyhow::anyhow;
use rocket::response::status::BadRequest;
use rocket::State;
use klyra_secrets::SecretStore;

#[rocket::get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    Ok(state.secret.clone())
}

struct MyState {
    secret: String,
}

// #[klyra_service::main]
pub async fn rocket(
    // #[klyra_secrets::Secrets] secret_store: SecretStore,
    secret_store: SecretStore,
) -> klyra_service::KlyraRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = if let Some(secret) = secret_store.get("MY_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let state = MyState { secret };
    let rocket = rocket::build()
        .mount("/", rocket::routes![secret])
        .manage(state);

    Ok(rocket)
}

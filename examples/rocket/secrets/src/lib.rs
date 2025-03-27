#[macro_use]
extern crate rocket;

use anyhow::anyhow;
use rocket::response::status::BadRequest;
use rocket::State;
use klyra_secrets::SecretStore;

#[get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    Ok(state.secret.clone())
}

struct MyState {
    secret: String,
}

#[klyra_service::main]
async fn rocket(
    #[klyra_secrets::Secrets] secret_store: SecretStore,
) -> klyra_service::KlyraRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = if let Some(secret) = secret_store.get("MY_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let state = MyState { secret };
    let rocket = rocket::build().mount("/", routes![secret]).manage(state);

    Ok(rocket)
}

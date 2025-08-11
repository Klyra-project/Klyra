// This will fail to compile since it's a library.

#[klyra_runtime::main]
async fn rocket() -> klyra_rocket::KlyraRocket {
    let rocket = rocket::build();
    Ok(rocket.into())
}

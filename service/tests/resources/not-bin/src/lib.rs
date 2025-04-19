// This will fail to compile since it's a library.

#[klyra_service::main]
async fn rocket() -> klyra_service::KlyraRocket {
    let rocket = rocket::build();
    Ok(rocket)
}

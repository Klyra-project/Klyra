#[klyra_service::main]
async fn rocket() -> klyra_service::KlyraRocket {
    let rocket = rocket::build();
    Ok(rocket)
}

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_service::main]
async fn rocket() -> Result<Rocket<Build>, klyra_service::Error> {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket)
}

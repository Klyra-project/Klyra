#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_service::main]
async fn rocket() -> klyra_service::KlyraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);
    Ok(rocket)
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_passes() {
        assert_eq!(super::index(), "Hello, world!");
    }
}

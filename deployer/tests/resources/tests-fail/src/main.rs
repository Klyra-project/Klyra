#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[klyra_runtime::main]
async fn rocket() -> klyra_rocket::KlyraRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);
    Ok(rocket.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_fails() {
        assert!(false);
    }
}

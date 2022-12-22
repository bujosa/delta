#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/profile")]
fn profile() -> &'static str {
    "Hello, profile!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, profile])
}

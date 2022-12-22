#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello, world!",
        },
    )
}

#[get("/profile")]
fn profile() -> &'static str {
    "Hello, profile!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, profile])
        .attach(Template::fairing())
}

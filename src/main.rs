mod model;

use rocket::{get, launch, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        .mount("/", routes![root])
}

#[get("/")]
async fn root() -> Template {
    Template::render("root", context! { message: "Hello, Rust"})
}
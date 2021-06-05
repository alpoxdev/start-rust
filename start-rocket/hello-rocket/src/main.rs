#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Rocket;
// use rocket_contrib::{templates::Template, serve::StaticFiles};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/status")]
fn status() -> &'static str{
	"API alive!"
}

fn rocket() -> Rocket{
	rocket::ignite().mount("/", routes![index, status])
}

fn main() {
	rocket().launch();
}
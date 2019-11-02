#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/ping")]
fn index() -> &'static str {
    "pong"
}

fn main() {
    rocket::ignite().mount("/metrix", routes![ping]).launch();
}

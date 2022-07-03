#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Rust - rocket: Hello Bro!"
}

fn main() {
    println!("Start the web server");
    rocket::ignite().mount("/", routes![index]).launch();
}

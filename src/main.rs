use dashmap::DashMap;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<String, String>::new())
        .mount("/", routes![index])
}

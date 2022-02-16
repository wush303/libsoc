#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Velkommen til libertær.dk - En verden af frihed"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

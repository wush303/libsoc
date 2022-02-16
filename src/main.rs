#[macro_use]
extern crate rocket;

use rocket::http::Cookie;
use rocket::http::CookieJar;
use uuid::Uuid;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> &'static str {
    match cookies
        .get("id")
        .map(|crumb| format!("Message: {}", crumb.value()))
    {
        Some(v) => println!("{}", v),
        None => cookies.add(Cookie::new("id", Uuid::new_v4().to_string())),
    }
    "Velkommen til libertær.dk - En verden af frihed"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

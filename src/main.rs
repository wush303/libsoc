#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::Cookie;
use rocket::http::CookieJar;

use uuid::Uuid;

#[derive(FromForm)]
struct Person<'r> {
    r#fname: &'r str,
    r#lname: &'r str,
}

#[post("/", data = "<person>")]
async fn bliv_aktiv(person: Form<Person<'_>>, cookies: &CookieJar<'_>) -> Option<NamedFile> {
    NamedFile::open("static/bliv_aktiv.html").await.ok()
}

#[get("/")]
async fn index(cookies: &CookieJar<'_>) -> Option<NamedFile> {
    match cookies
        .get("id")
        .map(|crumb| format!("Message: {}", crumb.value()))
    {
        Some(v) => println!("{}", v),
        None => cookies.add(Cookie::new("id", Uuid::new_v4().to_string())),
    }
    NamedFile::open("static/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/blivaktiv", routes![bliv_aktiv])
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}

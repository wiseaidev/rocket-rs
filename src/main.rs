#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::fs::{relative, NamedFile};
use rocket::form::Form;

#[derive(FromForm)]
struct LoginForm {
    username: String,
}

#[get("/")]
fn hello() -> &'static str {
    "rocket.rs"
}

#[get("/login")]
fn get_login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/login", &context)
}

#[post("/login", data = "<login_form>")]
fn post_login(login_form: Form<LoginForm>) -> Result<Redirect, String> {
    let username = &login_form.username;
    // info!("username: {:?}", username);
    println!("{:?}", username);
    Ok(Redirect::to("/"))
}

#[get("/signup")]
fn signup() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/signup", &context)
}

#[get("/forgot_password")]
fn forgotpassword() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/forgot_password", &context)
}

// Static files handler
#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static/")).join(file);
    NamedFile::open(path).await.ok()
}

#[catch(404)]
fn catch_404(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/404", &context)
}

#[catch(422)]
fn catch_422(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/422", &context)
}

#[catch(500)]
fn catch_500(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("pages/500", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            hello, files, get_login, post_login, signup, forgotpassword
        ])
        .attach(Template::fairing())
        .register("/", catchers![catch_404, catch_422, catch_500])
}

#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use std::collections::HashMap;



#[get("/")]
fn index() -> Template {
    let context:HashMap<String,String> = HashMap::new();
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
}
#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::time::{Instant};
use primes::{Sieve, PrimeSet};
use rocket::fs::{FileServer, relative};

struct PrimeReturn{
    ix : usize,
    n : u64
}

fn find_prime_after(start: u64) -> PrimeReturn{
    let mut pset = Sieve::new();
    let (ix, n) = pset.find(start);
    PrimeReturn{ix, n}
}

#[get("/")]
fn index() -> Template {
    let context:HashMap<String,String> = HashMap::new();
    Template::render("index", &context)
}
#[get("/prime-rust-backend/<start>")]
fn prime_rust_backend(start: u64) -> Template{
    let mut context:HashMap<String, String> = HashMap::new();
    let now = Instant::now();
    let returned_prime  = find_prime_after(start);
    let later = Instant::now();
    let elapsed_time = later - now;
    context.insert("time_span".to_owned(), (elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9).to_string());
    context.insert("ix".to_owned(), returned_prime.ix.to_string());
    context.insert("n".to_owned(), returned_prime.n.to_string());
    Template::render("prime_rust_backend", &context)
}

#[get("/prime-wasm-frontend/<start>")]
fn prime_wasm_frontend(start: u64) -> Template{
    let mut context:HashMap<String, u64> = HashMap::new();
    context.insert("start".to_owned(), start);
    Template::render("prime_wasm_frontend", &context)
}

#[get("/prime-js-frontend/<start>")]
fn prime_js_frontend(start: u64) -> Template{
    let context:HashMap<String, String> = HashMap::new();
    Template::render("prime_js_frontend", &context)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, prime_rust_backend, prime_wasm_frontend, prime_js_frontend])
    .mount("/static", FileServer::from(relative!("static")))
    .attach(Template::fairing())
}
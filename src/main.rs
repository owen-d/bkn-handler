#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
mod eddystone;
mod referrer;

use rocket_contrib::Template;
use eddystone::EddystoneUID;
use referrer::Referrer;
use rocket::response::Redirect;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, handle_impression, handle_passby])
        .attach(Template::fairing())
        .launch();
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
/*
Need to do a few things on request lifecycle
1) match beaconName & validate. If invalid (non hex, etc), use some 404 page
2) fetch beacon from cass. if none match, 404
3) inject cass code into html template
4) redirect
5) after-effect: increment count in cass.
 */

#[get("/bkn/<_name>", rank=2)]
fn handle_impression(_name: EddystoneUID, _referrer: Referrer) -> Redirect {
    Redirect::found("https://www.google.com")
}

#[get("/bkn/<_name>", rank=3)]
fn handle_passby(_name: EddystoneUID) -> Template {
    let context = TemplateContext {
        name: format!("placeholder"),
    };

    Template::render("bkn-redirect", &context)
}

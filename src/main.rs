#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
mod eddystone;
mod referrer;
mod cass;

use rocket_contrib::Template;
use eddystone::EddystoneUID;
use referrer::Referrer;
use rocket::response::Redirect;
use rocket::State;

fn main() {
    let conn = cass::Conn::new("127.0.0.1", 15).unwrap();
    rocket::ignite()
        .mount("/", routes![handle_impression, handle_passby])
        .attach(Template::fairing())
        .manage(conn)
        .launch();
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

/*
Need to do a few things on request lifecycle
1) match beaconName & validate. If invalid (non hex, etc), use some 404 page
3) redirect
4) drop 'passby metric in cass'
5) on hit when referrer = self, fetch beacon msg from cass
4) after-effect: increment count in cass.
 */

#[get("/bkn/<_name>", rank=2)]
fn handle_impression(_name: EddystoneUID, _referrer: Referrer, _conn: State<cass::Conn>) -> Redirect {
    Redirect::found("https://www.google.com")
}

#[get("/bkn/<_name>", rank=3)]
fn handle_passby(_name: EddystoneUID, _conn: State<cass::Conn>) -> Template {
    let context = TemplateContext {
        name: format!("placeholder"),
    };

    Template::render("bkn-redirect", &context)
}

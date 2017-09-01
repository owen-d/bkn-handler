#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod types;

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
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

// #[get("/bkn/<name>")]



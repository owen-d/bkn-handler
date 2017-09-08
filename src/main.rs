#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate scopeguard;
#[macro_use]
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

mod eddystone;
mod referrer;
mod cass;
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
mod config;

use errors::*;
use scopeguard::guard;
use rocket_contrib::Template;
use eddystone::EddystoneUID;
use referrer::{Referrer, AllowedReferrers};
use rocket::response::Redirect;
use rocket::State;

const DEFAULT_REDIRECT_URL: &'static str = "https://my.sharecro.ws";

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let conf = config::Env::new().load_env();
    let conn = cass::Conn::new(&format!("{}:{}", conf.cass_addr, conf.cass_port),
                               conf.cass_pool_size).chain_err(|| "failed to connect to cassandra")?;
    let allowed_refs = referrer::AllowedReferrers(conf.referrers.clone());
    Err(Error::with_chain(rocket::ignite()
                              .mount("/", routes![handle_impression, handle_passby])
                              .attach(Template::fairing())
                              .manage(conn)
                              .manage(allowed_refs)
                              .launch(),
                          "rocket error"))

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

#[get("/bkn/<name>", rank=2)]
fn handle_impression(name: EddystoneUID, _referrer: Referrer, conn: State<cass::Conn>) -> Redirect {
    conn.fetch_bkn_msg(&name)
        .map(|bkn| {
            let _ = conn.add_interaction(&bkn).unwrap_or(());
            if &bkn.msg_url == "" {
                Redirect::found(DEFAULT_REDIRECT_URL)
            } else {
                Redirect::found(&bkn.msg_url)
            }
        })
        .unwrap_or(Redirect::found(DEFAULT_REDIRECT_URL))
}

#[get("/bkn/<name>", rank=3)]
fn handle_passby(name: EddystoneUID, conn: State<cass::Conn>) -> Template {
    let context = TemplateContext { name: format!("placeholder") };

    let mut _conn = guard(conn, |conn| {
        conn.fetch_bkn_msg(&name)
            .and_then(|bkn| conn.add_passby(&bkn))
            .unwrap_or(())
    });
    Template::render("bkn-redirect", &context)
}

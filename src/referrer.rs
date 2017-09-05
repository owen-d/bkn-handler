use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

pub struct Referrer(String);

// note: mispelling of 'referrer' is actually correct:
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer
const HEADER: &'static str = "Referer";

impl<'a, 'r> FromRequest<'a, 'r> for Referrer {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Referrer, ()> {
        let keys: Vec<_> = request.headers().get(HEADER).collect();
        if keys.len() == 0 {
            return Outcome::Forward(())
        }

        let key = keys[0];

        return Outcome::Success(Referrer(key.to_string()));
    }
}

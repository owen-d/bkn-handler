use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub struct AllowedReferrers(pub Vec<String>);

impl AllowedReferrers {
    pub fn is_allowed(&self, referrer: &Referrer) -> bool {
        self.0.iter().fold(false, |acc, x| {
            if acc {
                acc
            }
            else if *x == "*" {
                true
            } else {
                *x == referrer.0
            }
        })
    }
}

pub struct Referrer(String);

// note: mispelling of 'referrer' is actually correct:
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer
const HEADER: &'static str = "Referer";

impl<'a, 'r> FromRequest<'a, 'r> for Referrer {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Referrer, ()> {
        let keys: Vec<_> = request.headers().get(HEADER).collect();
        if keys.len() == 0 {
            return Outcome::Forward(());
        }

        let key = keys[0];
        let referrer = Referrer(key.to_string());
        let allowed_referrers = request.guard::<State<AllowedReferrers>>()?;

        if allowed_referrers.is_allowed(&referrer) {
            return Outcome::Success(referrer);
        } else {
            return Outcome::Forward(());
        }

    }
}

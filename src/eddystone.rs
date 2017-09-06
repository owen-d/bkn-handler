extern crate hex;

use rocket::request::FromParam;
use rocket::http::RawStr;
use self::hex::FromHex;

const DEFAULT_NS: &str = "ecde1818fe845f2e3863";

pub struct EddystoneUID {
    ns: [u8; 10],
    id: [u8; 6],
}

impl EddystoneUID {
    fn new() -> Self {
        EddystoneUID {
            ns: [0;10],
            id: [0; 6],
        }
    }

    fn with_ns<'a, T>(&mut self, ns: T) -> &mut Self
        where T: Iterator<Item = &'a u8>
    {
        let mut new_ns = [0; 10];

        for (i, byte) in ns.take(10).enumerate() {
            new_ns[i] = *byte
        }
        self.ns = new_ns;
        self

    }

    fn with_id<'a, T>(&mut self, id: T) -> &mut Self
        where T: Iterator<Item = &'a u8>
    {
        let mut new_id = [0; 6];
        for (i, byte) in id.take(6).enumerate() {
            new_id[i] = *byte
        }

        self.id = new_id;
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [&self.ns.to_vec()[..], &self.id.to_vec()[..]].concat()
    }
}

impl<'r> FromParam<'r> for EddystoneUID {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param.url_decode()
            .map_err(|_| param)
            .and_then(|s| Vec::from_hex(s).map_err(|_| param))
            .and_then(|x| {
                match x.len() {
                    // full uid (namespace + id)
                    16 => {
                        let ns = x.iter().take(10);
                        let id = x.iter().skip(10).take(6);
                        let mut eddy = EddystoneUID::new();
                        eddy.with_ns(ns).with_id(id);
                        Ok(eddy)
                    },
                    // just uid
                    6 => {
                        let mut eddy = EddystoneUID::new();
                        eddy.with_ns(Vec::from_hex(DEFAULT_NS).unwrap().iter().take(10));
                        eddy.with_id(x.iter().take(6));
                        Ok(eddy)
                    },
                    _ => Err(param),
                }
            })
    }
}

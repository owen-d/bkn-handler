extern crate rocket;
// extern crate serialize;

use rocket::request::FromParam;
use rocket::http::RawStr;
// use serialize::hex::ToHex;

struct EddystoneUID {
    ns: Option<[u8; 10]>,
    id: [u8; 6],
}

impl EddystoneUID {
    fn new() -> Self {
        EddystoneUID {
            ns: None,
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
        self.ns = Some(new_ns);
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
}

impl<'r> FromParam<'r> for EddystoneUID {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error>{
        param.url_decode()
            .map(|s| s.into_bytes())
            .map(|x| {
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
                    // 6 => 1,
                    _ => Err(param),
                }
            })
            .map_err(|_| Err(param))
    }

}

// impl<'r> FromParam<'r> for MyParam<'r> {
//     type Error = &'r RawStr;

//     fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
//         let (key, val_str) = match param.find(':') {
//             Some(i) if i > 0 => (&param[..i], &param[(i + 1)..]),
//             _ => return Err(param)
//         };

//         if !key.chars().all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
//             return Err(param);
//         }

//         val_str.parse().map(|value| {
//             MyParam {
//                 key: key,
//                 value: value
//             }
//         }).map_err(|_| param)
//     }
// }

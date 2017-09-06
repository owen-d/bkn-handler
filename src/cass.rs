// fn to fetch record (will be called twice, first on initial call & again on redirect.)
// fn to add an impression
// fn to add a pass-by (google bot interaction)
extern crate cdrs;
extern crate r2d2;
extern crate uuid;

use eddystone::EddystoneUID;
use self::cdrs::authenticators::NoneAuthenticator;
use self::cdrs::transport::TransportTcp;
use self::cdrs::connection_manager::ConnectionManager;
use self::cdrs::compression::Compression;
use self::cdrs::query::QueryBuilder;
use self::cdrs::types::value::Value;
use self::cdrs::types::IntoRustByName;

use self::uuid::Uuid;
use std::error::Error;
use std::convert::Into;

pub struct Conn(r2d2::Pool<ConnectionManager<NoneAuthenticator, TransportTcp>>);

impl Conn {
    pub fn new(addr: &str, size: u32) -> Result<Self, &str> {
        let config = r2d2::Config::builder()
            .pool_size(size)
            .build();
        let transport = TransportTcp::new(addr).unwrap();
        let authenticator = NoneAuthenticator;
        let manager = ConnectionManager::new(transport, authenticator, Compression::None);

        r2d2::Pool::new(config, manager)
            .map_err(|e| e.description())
            .map(|pool| Conn(pool))
    }

    pub fn fetch_bkn_msg(&self, eddy: &EddystoneUID) -> Result<Beacon, &str> {
        let pool = self.0.clone();
        let values: Vec<Value> = vec![eddy.to_vec().into()];
        let query = QueryBuilder::new("SELECT name, msg_url, user_id FROM bkn.beaons_by_id WHERE \
                                       name = ? LIMIT 1;")
            .values(values)
            .finalize();

        let body = pool.get()
            .map_err(|e| e.description())
            .and_then(|conn| {
                conn.query(query, false, false)
                    .map_err(|e| e.description())
            })
            .and_then(|res| {
                res.get_body()
                    .map_err(|e| e.description())
            })?;

        let none_matched = "no beacons matched";

        body.into_rows()
            .ok_or(none_matched)
            .and_then(|rows| {
                let bkns: Vec<Beacon> = rows.iter()
                    .map(|row| {
                        let mut bkn = Beacon { ..Default::default() };
                        if let Ok(name) = row.get_r_by_name("name") {
                            bkn.name = name;
                        }


                        if let Ok(msg_url) = row.get_r_by_name("msg_url") {
                            bkn.msg_url = msg_url;
                        }

                        if let Ok(user_id) = row.get_r_by_name("user_id") {
                            let _: Uuid = user_id;
                            // bkn.user_id = user_id;
                        }

                        bkn
                    })
                    .take(1)
                    .collect();
                match bkns.len() {
                    0 => Err(none_matched),
                    _ => Ok(bkns.remove(0)),
                }
            })
    }
    // pub fn add_passby(){}
    // pub fn add_interaction(){}
}

#[derive(Debug, Default)]
pub struct Beacon {
    pub user_id: Uuid,
    pub name: Vec<u8>,
    pub msg_url: String,
}

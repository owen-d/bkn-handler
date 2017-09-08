// fn to fetch record (will be called twice, first on initial call & again on redirect.)
// fn to add an impression
// fn to add a pass-by (google bot interaction)
extern crate cdrs;
extern crate r2d2;
extern crate uuid;
extern crate time;
extern crate rocket;

use eddystone::EddystoneUID;
use self::cdrs::authenticators::NoneAuthenticator;
use self::cdrs::transport::TransportTcp;
use self::cdrs::connection_manager::ConnectionManager;
use self::cdrs::compression::Compression;
use self::cdrs::query::QueryBuilder;
use self::cdrs::types::value::{Value, Bytes};
use self::cdrs::types::IntoRustByName;
use self::rocket::request::{self, FromRequest};
use self::rocket::{Request, State, Outcome};
use self::uuid::Uuid;
use std::convert::Into;
use super::errors::*;

pub struct Conn(r2d2::Pool<ConnectionManager<NoneAuthenticator, TransportTcp>>);

impl Conn {
    pub fn new(addr: &str, size: u32) -> Result<Self> {
        let config = r2d2::Config::builder()
            .pool_size(size)
            .build();
        let transport = TransportTcp::new(addr).chain_err(|| "failed to connect")?;
        let authenticator = NoneAuthenticator;
        let manager = ConnectionManager::new(transport, authenticator, Compression::None);

        r2d2::Pool::new(config, manager)
            .map(|pool| Conn(pool))
            .chain_err(|| "Failed to initialize pool")
    }

    pub fn fetch_bkn_msg(&self, eddy: &EddystoneUID) -> Result<Beacon> {
        let pool = self.0.clone();
        let values: Vec<Value> = vec![Bytes::new(eddy.to_vec()).into()];
        let query = QueryBuilder::new("SELECT name, msg_url, user_id, deploy_name FROM \
                                       bkn.beacons_by_id WHERE name = ? LIMIT 1;")
            .values(values)
            .finalize();

        let body = pool.get()
            .chain_err(|| "failed pool initialization")
            .and_then(|mut conn| {
                conn.query(query, false, false)
                    .chain_err(|| "failed query")
            })
            .and_then(|query| {
                query.get_body()
                    .chain_err(|| "failed to get body")
            })?;

        let none_matched = "no beacons matched";

        body.into_rows()
            .ok_or(none_matched.into())
            .and_then(|rows| {
                let mut bkns: Vec<Beacon> = rows.iter()
                    .map(|row| {
                        let mut bkn = Beacon { ..Default::default() };
                        if let Ok(name) = row.get_r_by_name("name") {
                            bkn.name = name;
                        }

                        if let Ok(msg_url) = row.get_r_by_name("msg_url") {
                            bkn.msg_url = msg_url;
                        }

                        if let Ok(user_id) = row.get_r_by_name("user_id") {
                            bkn.user_id = user_id;
                        }

                        if let Ok(deploy_name) = row.get_r_by_name("deploy_name") {
                            bkn.deploy_name = deploy_name;
                        }

                        bkn
                    })
                    .take(1)
                    .collect();
                match bkns.len() {
                    0 => Err(none_matched.into()),
                    _ => Ok(bkns.remove(0)),
                }
            })
    }

    pub fn add_passby(&self, bkn: &Beacon) -> Result<()> {
        self.add_impression("bkn", "passerby", bkn)
    }
    pub fn add_interaction(&self, bkn: &Beacon) -> Result<()> {
        self.add_impression("bkn", "interactions", bkn)
    }

    fn add_impression(&self, keyspace: &str, table: &str, bkn: &Beacon) -> Result<()> {
        let pool = self.0.clone();
        let now = time::get_time();
        let values: Vec<Value> = vec![Bytes::new(bkn.name.clone()).into(),
                                      bkn.deploy_name.clone().into(),
                                      now.into(),
                                      bkn.user_id.into()];
        let querystring = format!("INSERT INTO {}.{} (bkn_name, deploy_name, moment, \
                                   bkn_user_id) VALUES (?, ?, ?, ?);",
                                  keyspace,
                                  table);
        let query = QueryBuilder::new(querystring)
            .values(values)
            .finalize();

        pool.get()
            .chain_err(|| "failed to acquire a connection")
            .and_then(|mut conn| {
                conn.query(query, false, false)
                    .map(|_| ())
                    .chain_err(|| "failed query")
            })
    }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Conn>>()?;
        Outcome::Success(Conn(pool.0.clone()))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Beacon {
    pub user_id: Uuid,
    pub name: Vec<u8>,
    pub msg_url: String,
    pub deploy_name: String,
}

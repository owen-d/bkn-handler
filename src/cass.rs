// fn to fetch record (will be called twice, first on initial call & again on redirect.)
// fn to add an impression
// fn to add a pass-by (google bot interaction)
extern crate cdrs;
extern crate r2d2;

use cdrs::client::CDRS;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::transport::TransportTcp;
use cdrs::connection_manager::ConnectionManager;
use cdrs::compression::Compression;
use cdrs::query::QueryBuilder;
use uuid::Uuid;

pub struct Conn(r2d2::Pool<ConnectionManager>);

impl Conn {
    pub fn new(addr: &str, size: int8) -> Result<Self, &str> {
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

    pub fn fetch_bkn_msg(&self) -> Result<Beacon, &str> {
        let pool = self.0.clone();
        let values: Vec<Value> = vec![];
        let query = QueryBuilder::new("SELECT name, msg_url FROM bkn.beaons_by_id WHERE user_id = ? LIMIT 1;").values(values).finalize();
        let mut conn = pool.get()?;

        let res = conn.query(query, false, false)?;
        let res_body = res.get_body().unwrap();

        if let Some(rows) = res_body.into_rows() {
            let bkns: Vec<Beacon> = rows.iter()
                .map(|row| {
                    let mut bkn = Beacon{..Default::default()};
                })
        }
    }
    // pub fn add_passby(){}
    // pub fn add_interaction(){}
}

#[derive(Debug, Default)]
pub struct Beacon {
    pub user_id: uuid,
    pub name: Vec<u8>,
    pub msg_url: String,
}

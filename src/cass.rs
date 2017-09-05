// fn to fetch record (will be called twice, first on initial call & again on redirect.)
// fn to add an impression
// fn to add a pass-by (google bot interaction)
extern crate cdrs;
extern crate r2d2;

use cdrs::client::CDRS;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::transport::TransportTcp;
use cdrs::connection_manager::ConnectionManager;


pub fn new(addr: &str, size: int8) -> Result<r2d2::Pool<ConnectionManager>> {
    let config = r2d2::Config::builder()
        .pool_size(size)
        .build();
    let transport = TransportTcp::new(addr).unwrap();
    let authenticator = NoneAuthenticator;
    let manager = ConnectionManager::new(transport, authenticator, Compression::None);

    r2d2::Pool::new(config, manager)
}

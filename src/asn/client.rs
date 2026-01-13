// ASN lookup trait

use super::types::AsnInfo;
use std::io::Error;
use std::net::IpAddr;

pub trait Asn {
    fn lookup_asn(&self, ip: IpAddr) -> Result<Vec<AsnInfo>, Error>;
}

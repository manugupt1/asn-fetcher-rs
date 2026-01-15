// ASN lookup trait

use super::types::AsnInfo;
use std::io::Error;
use std::net::IpAddr;

/// Trait for ASN lookup providers
///
/// Implement this trait to provide ASN lookup functionality
/// from different data sources (RIPE, ARIN, etc.)
pub trait Asn {
    /// Looks up ASN information for a given IP address
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address to look up (IPv4 or IPv6)
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup fails due to network issues,
    /// API errors, or invalid response data
    fn lookup_asn(&self, ip: IpAddr) -> Result<Vec<AsnInfo>, Error>;
}

pub fn map_reqwest_error(err: reqwest::Error) -> Error {
    if err.is_timeout() {
        Error::new(std::io::ErrorKind::TimedOut, err.to_string())
    } else if err.is_connect() {
        Error::new(std::io::ErrorKind::ConnectionRefused, err.to_string())
    } else {
        Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

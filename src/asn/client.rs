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

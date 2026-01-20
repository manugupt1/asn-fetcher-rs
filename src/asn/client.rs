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

/// Creates a standardized error message with provider name prefix
///
/// # Arguments
///
/// * `provider` - The name of the ASN provider (e.g., "Ripe", "IPApi", "TeamCymruWhois")
/// * `message` - The error message to format
///
/// # Returns
///
/// A formatted error message in the format: `[ProviderName] Error message`
pub fn format_provider_error(provider: &str, message: &str) -> String {
    format!("[{}] {}", provider, message)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_provider_error() {
        let error = format_provider_error("Ripe", "Missing 'data' field in API response");
        assert_eq!(error, "[Ripe] Missing 'data' field in API response");
    }

    #[test]
    fn test_format_provider_error_ipapi() {
        let error = format_provider_error("IPApi", "API error: Rate limit exceeded");
        assert_eq!(error, "[IPApi] API error: Rate limit exceeded");
    }

    #[test]
    fn test_format_provider_error_teamcymru() {
        let error = format_provider_error("TeamCymruWhois", "Command failed: whois not installed");
        assert_eq!(
            error,
            "[TeamCymruWhois] Command failed: whois not installed"
        );
    }

    #[test]
    fn test_format_provider_error_empty_message() {
        let error = format_provider_error("TestProvider", "");
        assert_eq!(error, "[TestProvider] ");
    }
}

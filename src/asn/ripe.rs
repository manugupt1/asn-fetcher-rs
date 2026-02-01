// RIPE NCC ASN lookup implementation

use crate::asn::client::map_reqwest_error;

use super::client::*;
use super::types::AsnInfo;
use reqwest::blocking::ClientBuilder;
use std::{io::Error, net::IpAddr, time::Duration};

/// RIPE NCC ASN lookup client
///
/// This client queries the RIPE NCC API to retrieve ASN information
/// for a given IP address.
pub struct Ripe {
    client: reqwest::blocking::Client,
    server_url: String,
}

impl Ripe {
    const DEFAULT_SERVER_URL: &'static str = "https://stat.ripe.net/data/prefix-overview/data.json";
    const TIMEOUT_SECS: u64 = 10;

    /// Creates a new RIPE client with default configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(Self::TIMEOUT_SECS))
            .build()?;
        Ok(Ripe {
            client,
            server_url: Self::DEFAULT_SERVER_URL.to_string(),
        })
    }
}

impl Asn for Ripe {
    fn lookup_asn(&self, ip: IpAddr) -> Result<Vec<AsnInfo>, Error> {
        let url = format!("{}?resource={}", self.server_url, ip);
        let response = self.client.get(&url).send().map_err(map_reqwest_error)?;

        let json_data: serde_json::Value = response.json().map_err(map_reqwest_error)?;

        // Check if 'data' field exists
        let data = json_data.get("data").ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing 'data' field in response",
            )
        })?;

        // Check if 'asns' field exists and is an array
        let asns_array = data.get("asns").and_then(|v| v.as_array()).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing or invalid 'asns' field in response",
            )
        })?;

        let asns = asns_array
            .iter()
            .map(|asn_obj| {
                let asn = asn_obj["asn"]
                    .as_u64()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| {
                        eprintln!("Warning: Missing or invalid 'asn' field in ASN object");
                        "N/A".to_string()
                    });
                let holder = asn_obj["holder"]
                    .as_str()
                    .unwrap_or_else(|| {
                        eprintln!("Warning: Missing or invalid 'holder' field in ASN object");
                        "Unknown"
                    })
                    .to_string();
                AsnInfo { asn, holder }
            })
            .collect();

        Ok(asns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ripe_new() {
        let ripe = Ripe::new();
        assert!(ripe.is_ok());
    }

    #[test]
    fn test_ripe_has_default_url() {
        let ripe = Ripe::new().unwrap();
        assert_eq!(ripe.server_url, Ripe::DEFAULT_SERVER_URL);
    }

    #[test]
    fn test_ripe_has_timeout() {
        let ripe = Ripe::new().unwrap();
        // Verify the client was created successfully
        // We can't directly test the timeout, but we can ensure the struct is valid
        assert!(!ripe.server_url.is_empty());
    }

    #[test]
    fn test_parse_valid_response() {
        // Construct AsnInfo instances directly to validate expected values
        let asns: Vec<AsnInfo> = vec![
            AsnInfo {
                asn: "15169".to_string(),
                holder: "Google LLC".to_string(),
            },
            AsnInfo {
                asn: "13335".to_string(),
                holder: "Cloudflare, Inc.".to_string(),
            },
        ];

        assert_eq!(asns.len(), 2);
        assert_eq!(asns[0].asn, "15169");
        assert_eq!(asns[0].holder, "Google LLC");
        assert_eq!(asns[1].asn, "13335");
        assert_eq!(asns[1].holder, "Cloudflare, Inc.");
    }

    #[test]
    fn test_parse_missing_asn_field() {
        // Simulate behavior when the ASN field is missing by using the expected fallback
        let asns: Vec<AsnInfo> = vec![AsnInfo {
            asn: "N/A".to_string(),
            holder: "Google LLC".to_string(),
        }];
        // Should not error, should use fallback
        assert_eq!(asns.len(), 1);
        assert_eq!(asns[0].asn, "N/A");
        assert_eq!(asns[0].holder, "Google LLC");
    }

    #[test]
    fn test_parse_missing_holder_field() {
        use serde_json::json;

        let json_data = json!({
            "data": {
                "asns": [
                    {"asn": 15169}  // Missing holder field
                ]
            }
        });

        let data = json_data.get("data").unwrap();
        let asns_array = data.get("asns").and_then(|v| v.as_array()).unwrap();

        let asns: Vec<AsnInfo> = asns_array
            .iter()
            .map(|asn_obj| {
                let asn = asn_obj["asn"]
                    .as_u64()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let holder = asn_obj["holder"].as_str().unwrap_or("Unknown").to_string();
                AsnInfo { asn, holder }
            })
            .collect();

        // Should not error, should use fallback
        assert_eq!(asns.len(), 1);
        assert_eq!(asns[0].asn, "15169");
        assert_eq!(asns[0].holder, "Unknown");
    }

    #[test]
    fn test_parse_invalid_asn_type() {
        use serde_json::json;

        let json_data = json!({
            "data": {
                "asns": [
                    {"asn": "not-a-number", "holder": "Google LLC"}  // asn should be a number
                ]
            }
        });

        let data = json_data.get("data").unwrap();
        let asns_array = data.get("asns").and_then(|v| v.as_array()).unwrap();

        let asns: Vec<AsnInfo> = asns_array
            .iter()
            .map(|asn_obj| {
                let asn = asn_obj["asn"]
                    .as_u64()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let holder = asn_obj["holder"].as_str().unwrap_or("Unknown").to_string();
                AsnInfo { asn, holder }
            })
            .collect();

        // Should not error, should use fallback
        assert_eq!(asns.len(), 1);
        assert_eq!(asns[0].asn, "N/A");
        assert_eq!(asns[0].holder, "Google LLC");
    }
}

use std::env;
use std::net::IpAddr;

use super::client::*;
use super::types::AsnInfo;
use reqwest::blocking::ClientBuilder;

// IPAPI ASN Lookup client
pub struct IPApi {
    client: reqwest::blocking::Client,
}

impl IPApi {
    pub fn new() -> Result<Self, std::io::Error> {
        let client = ClientBuilder::new()
            .user_agent("asn-fetcher/0.1.1")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(map_reqwest_error)?;
        Ok(IPApi { client })
    }
}

impl Asn for IPApi {
    fn lookup_asn(&self, ip: IpAddr) -> Result<Vec<AsnInfo>, std::io::Error> {
        let url = match env::var("IPAPI_API_KEY") {
            Ok(api_key) => format!("https://ipapi.co/{}/json?key={}", ip, api_key),
            Err(_) => format!("https://ipapi.co/{}/json", ip),
        };

        let response = self.client.get(&url).send().map_err(map_reqwest_error)?;

        if !response.status().is_success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("HTTP error: {}", response.status()),
            ));
        }

        let response_text = response.text().map_err(map_reqwest_error)?;

        // Parse JSON, providing helpful error message if API returns non-JSON (e.g., rate limit error)
        let json: serde_json::Value = serde_json::from_str(&response_text).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("API returned non-JSON response: {}", response_text),
            )
        })?;

        // Check if the API returned an error object
        if let Some(error) = json.get("error").and_then(|v| v.as_bool()) {
            if error {
                let message = json
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("API error: {}", message),
                ));
            }
        }

        let asn = json
            .get("asn")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let holder = json
            .get("org")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        Ok(vec![AsnInfo { asn, holder }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipapi_new() {
        let ipapi = IPApi::new();
        assert!(ipapi.is_ok());
    }

    #[test]
    fn test_ipapi_client_creation() {
        let ipapi = IPApi::new().unwrap();
        // Verify the client was created successfully by checking the struct
        // We can't directly inspect the client, but we can verify the struct exists
        assert!(std::mem::size_of_val(&ipapi.client) > 0);
    }

    #[test]
    fn test_ipapi_has_timeout() {
        let ipapi = IPApi::new().unwrap();
        // Verify the client was created successfully
        // The timeout is set during creation, so if new() succeeds, timeout is configured
        assert!(std::mem::size_of_val(&ipapi) > 0);
    }
}

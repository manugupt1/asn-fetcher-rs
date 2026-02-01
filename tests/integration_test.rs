// Integration tests for ASN provider error scenarios
//
// This file contains comprehensive error handling tests for all ASN providers:
// - Ripe (RIPE NCC API)
// - IPApi (ipapi.co API)
// - TeamCymruWhois (Team Cymru whois)
//
// Tests are divided into two categories:
// 1. Tests that run on CI (using mocked HTTP responses)
// 2. Tests that require real network/system access (marked with #[ignore])
//
// To run all tests including ignored ones:
//   cargo test -- --ignored --test-threads=1
//
// To run only CI-safe tests:
//   cargo test

use asn_fetcher::asn::{Asn, IPApi, Ripe, TeamCymruWhois};
use std::net::{IpAddr, Ipv4Addr};

// ============================================================================
// RIPE Provider Error Tests
// ============================================================================

#[cfg(test)]
mod ripe_error_tests {
    use super::*;
    use mockito::{Server, ServerGuard};

    /// Helper function to create a mock server and Ripe client
    fn setup_mock_ripe(timeout_secs: u64) -> (ServerGuard, Ripe) {
        let server = Server::new();
        let url = server.url();
        let ripe = Ripe::with_config(url, timeout_secs).expect("Failed to create Ripe client");
        (server, ripe)
    }

    #[test]
    fn test_ripe_malformed_json_response() {
        let (mut server, ripe) = setup_mock_ripe(10);

        // Mock server returns invalid JSON
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{ invalid json }")
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        assert!(result.is_err(), "Should fail on malformed JSON");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
        // The error message may vary based on reqwest/serde version
        let err_msg = err.to_string();
        assert!(
            err_msg.contains("expected") || err_msg.contains("EOF") || err_msg.contains("decoding"),
            "Error should indicate JSON parsing issue: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ripe_missing_data_field() {
        let (mut server, ripe) = setup_mock_ripe(10);

        // Mock server returns JSON without 'data' field
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "ok"}"#)
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        assert!(result.is_err(), "Should fail when 'data' field is missing");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("Missing 'data' field"),
            "Error message should mention missing 'data' field: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ripe_missing_asns_field() {
        let (mut server, ripe) = setup_mock_ripe(10);

        // Mock server returns JSON without 'asns' field
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data": {"prefix": "8.8.8.0/24"}}"#)
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        assert!(result.is_err(), "Should fail when 'asns' field is missing");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("Missing or invalid 'asns' field"),
            "Error message should mention missing 'asns' field: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ripe_http_error_404() {
        let (mut server, ripe) = setup_mock_ripe(10);

        // Mock server returns 404 Not Found
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(404)
            .with_body("Not Found")
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        // reqwest returns the response even for 404, then fails on JSON parsing
        assert!(result.is_err(), "Should fail on HTTP 404 error");

        mock.assert();
    }

    #[test]
    fn test_ripe_http_error_500() {
        let (mut server, ripe) = setup_mock_ripe(10);

        // Mock server returns 500 Internal Server Error
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        // reqwest returns the response even for 500, then fails on JSON parsing
        assert!(result.is_err(), "Should fail on HTTP 500 error");

        mock.assert();
    }

    #[test]
    fn test_ripe_connection_refused() {
        // Use an invalid port that won't be listening
        let ripe = Ripe::with_config("http://127.0.0.1:1".to_string(), 2)
            .expect("Failed to create Ripe client");

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        assert!(result.is_err(), "Should fail when connection is refused");
        let err = result.unwrap_err();
        // Could be ConnectionRefused or TimedOut depending on the system
        assert!(
            matches!(
                err.kind(),
                std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::TimedOut
            ),
            "Should be ConnectionRefused or TimedOut, got: {:?}",
            err.kind()
        );
    }

    /// Test network timeout - requires real network, so ignored by default
    ///
    /// This test uses a non-routable IP address (10.255.255.1) which should
    /// cause a timeout.
    #[test]
    #[ignore]
    fn test_ripe_network_timeout() {
        // Use a very short timeout to force timeout behavior
        let ripe = Ripe::with_config("http://10.255.255.1:9999".to_string(), 1)
            .expect("Failed to create Ripe client");

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ripe.lookup_asn(ip);

        assert!(result.is_err(), "Should timeout");
        let err = result.unwrap_err();
        assert_eq!(
            err.kind(),
            std::io::ErrorKind::TimedOut,
            "Should be TimedOut error"
        );
    }
}

// ============================================================================
// IPApi Provider Error Tests
// ============================================================================

#[cfg(test)]
mod ipapi_error_tests {
    use super::*;
    use mockito::{Server, ServerGuard};

    /// Helper function to create a mock server and IPApi client
    fn setup_mock_ipapi(timeout_secs: u64) -> (ServerGuard, IPApi) {
        let server = Server::new();
        let url = server.url();
        let ipapi = IPApi::with_config(url, timeout_secs).expect("Failed to create IPApi client");
        (server, ipapi)
    }

    #[test]
    fn test_ipapi_malformed_json_response() {
        let (mut server, ipapi) = setup_mock_ipapi(10);

        // Mock server returns invalid JSON
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("not valid json at all")
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should fail on malformed JSON");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("API returned non-JSON response"),
            "Error should indicate non-JSON response: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ipapi_rate_limit_error() {
        let (mut server, ipapi) = setup_mock_ipapi(10);

        // Mock server returns rate limit error (429)
        // IPApi typically returns an error JSON object
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": true, "reason": "RateLimited"}"#)
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should fail on rate limit");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
        assert!(
            err.to_string().contains("RateLimited"),
            "Error should mention rate limiting: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ipapi_api_error_response() {
        let (mut server, ipapi) = setup_mock_ipapi(10);

        // Mock server returns an API error
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": true, "reason": "Invalid IP address"}"#)
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should fail on API error response");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
        assert!(
            err.to_string().contains("Invalid IP address"),
            "Error should contain API error reason: {}",
            err
        );

        mock.assert();
    }

    #[test]
    fn test_ipapi_http_error_500() {
        let (mut server, ipapi) = setup_mock_ipapi(10);

        // Mock server returns 500 Internal Server Error
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should fail on HTTP 500 error");
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);

        mock.assert();
    }

    #[test]
    fn test_ipapi_connection_refused() {
        // Use an invalid port that won't be listening
        let ipapi = IPApi::with_config("http://127.0.0.1:1".to_string(), 2)
            .expect("Failed to create IPApi client");

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should fail when connection is refused");
        let err = result.unwrap_err();
        assert!(
            matches!(
                err.kind(),
                std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::TimedOut
            ),
            "Should be ConnectionRefused or TimedOut, got: {:?}",
            err.kind()
        );
    }

    /// Test network timeout - requires real network, so ignored by default
    #[test]
    #[ignore]
    fn test_ipapi_network_timeout() {
        // Use a very short timeout to force timeout behavior
        let ipapi = IPApi::with_config("http://10.255.255.1:9999".to_string(), 1)
            .expect("Failed to create IPApi client");

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        assert!(result.is_err(), "Should timeout");
        let err = result.unwrap_err();
        assert_eq!(
            err.kind(),
            std::io::ErrorKind::TimedOut,
            "Should be TimedOut error"
        );
    }

    #[test]
    fn test_ipapi_missing_asn_field() {
        let (mut server, ipapi) = setup_mock_ipapi(10);

        // Mock server returns JSON without asn field
        let mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ip": "8.8.8.8", "city": "Mountain View"}"#)
            .create();

        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = ipapi.lookup_asn(ip);

        // Should succeed but return "Unknown" for missing fields
        assert!(result.is_ok(), "Should succeed even with missing ASN field");
        let asns = result.unwrap();
        assert_eq!(asns.len(), 1);
        assert_eq!(asns[0].asn, "Unknown");

        mock.assert();
    }
}

// ============================================================================
// TeamCymruWhois Provider Error Tests
// ============================================================================

#[cfg(test)]
mod teamcymru_error_tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_teamcymru_missing_whois_command() {
        // This test requires the whois command to NOT be available
        // It's difficult to test reliably without mocking the command execution
        // This is left as an ignored test to be run manually in environments
        // where whois is not installed

        let whois = TeamCymruWhois::default();
        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let result = whois.lookup_asn(ip);

        // If whois is not installed, we should get an error
        if result.is_err() {
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::Other);
            assert!(
                err.to_string().contains("whois") || err.to_string().contains("installed"),
                "Error should mention whois or installation: {}",
                err
            );
        }
    }

    #[test]
    #[ignore]
    fn test_teamcymru_whois_timeout() {
        // This test requires actual network access to Team Cymru's whois server
        // It's marked as ignored because it depends on external network resources

        let whois = TeamCymruWhois::default();
        // Use a reserved/bogon IP that might cause issues
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let result = whois.lookup_asn(ip);

        // The result behavior depends on whois implementation and network conditions
        // We just verify it doesn't panic
        let _ = result;
    }

    #[test]
    #[ignore]
    fn test_teamcymru_private_ip() {
        // This test requires actual whois command
        // Private IPs may not have ASN information

        let whois = TeamCymruWhois::default();
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let result = whois.lookup_asn(ip);

        // Private IPs typically return no ASN data or an error
        // We just verify the behavior is consistent
        if result.is_ok() {
            let asns = result.unwrap();
            // May be empty or contain special ASN for private ranges
            assert!(asns.len() <= 1);
        }
    }
}

// ============================================================================
// Cross-Provider Tests
// ============================================================================

#[cfg(test)]
mod cross_provider_tests {
    use super::*;

    #[test]
    fn test_all_providers_creation() {
        // Verify that all providers can be created successfully
        let ripe = Ripe::new();
        assert!(ripe.is_ok(), "Ripe provider should be creatable");

        let ipapi = IPApi::new();
        assert!(ipapi.is_ok(), "IPApi provider should be creatable");

        let _whois = TeamCymruWhois::default();
        // TeamCymruWhois doesn't have a constructor that can fail
    }
}

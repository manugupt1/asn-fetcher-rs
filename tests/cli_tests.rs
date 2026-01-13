// Integration tests for the CLI

use asn_parser::asn::{Asn, Ripe};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Integration test for IPv4 ASN lookup
///
/// Note: This test makes a real network call to the RIPE API.
/// Run with: cargo test -- --ignored
#[test]
#[ignore]
fn test_ripe_lookup_ipv4() {
    let ripe = Ripe::new().expect("Failed to create Ripe client");
    let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));

    let result = ripe.lookup_asn(ip);
    assert!(result.is_ok(), "ASN lookup should succeed for 8.8.8.8");

    let asns = result.unwrap();
    assert!(!asns.is_empty(), "Should return at least one ASN for 8.8.8.8");
}

/// Integration test for IPv6 ASN lookup
///
/// Note: This test makes a real network call to the RIPE API.
/// Run with: cargo test -- --ignored
#[test]
#[ignore]
fn test_ripe_lookup_ipv6() {
    let ripe = Ripe::new().expect("Failed to create Ripe client");
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888));

    let result = ripe.lookup_asn(ip);
    assert!(result.is_ok(), "ASN lookup should succeed for IPv6 address");

    let asns = result.unwrap();
    assert!(!asns.is_empty(), "Should return at least one ASN");
}

#[test]
fn test_ripe_client_creation() {
    let result = Ripe::new();
    assert!(result.is_ok(), "Should be able to create a Ripe client");
}

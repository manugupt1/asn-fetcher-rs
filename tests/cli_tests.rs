// Integration tests for the CLI

use asn_fetcher::asn::{Asn, Ripe};
use assert_cmd::prelude::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

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
    assert!(
        !asns.is_empty(),
        "Should return at least one ASN for 8.8.8.8"
    );
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

#[test]
fn test_valid_ipv4() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("asn-fetcher"));
    cmd.env("ASN_FETCHER_SKIP_LOOKUP", "1");
    cmd.arg("127.0.0.1");
    cmd.assert().success();
}

#[test]
fn test_valid_ipv6() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("asn-fetcher"));
    cmd.env("ASN_FETCHER_SKIP_LOOKUP", "1");
    cmd.arg("::1");
    cmd.assert().success();
}

#[test]
fn test_invalid_ip() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("asn-fetcher"));
    cmd.arg("not-an-ip");
    cmd.assert().failure();
}

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_valid_ipv4() {
    let mut cmd = Command::cargo_bin("asn-fetcher").unwrap();
    cmd.arg("127.0.0.1");
    cmd.assert().success();
}

#[test]
fn test_valid_ipv6() {
    let mut cmd = Command::cargo_bin("asn-fetcher").unwrap();
    cmd.arg("::1");
    cmd.assert().success();
}

#[test]
fn test_invalid_ip() {
    let mut cmd = Command::cargo_bin("asn-fetcher").unwrap();
    cmd.arg("not-an-ip");
    cmd.assert().failure();
}

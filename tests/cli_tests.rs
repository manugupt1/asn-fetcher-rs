use assert_cmd::prelude::*;
use assert_cmd::cargo::cargo_bin;

#[test]
fn test_valid_ipv4() {
    let mut cmd = cargo_bin!("asn-fetcher");
    cmd.arg("127.0.0.1");
    cmd.assert().success();
}

#[test]
fn test_valid_ipv6() {
    let mut cmd = cargo_bin!("asn-fetcher");
    cmd.arg("::1");
    cmd.assert().success();
}

#[test]
fn test_invalid_ip() {
    let mut cmd = cargo_bin!("asn-fetcher");
    cmd.arg("not-an-ip");
    cmd.assert().failure();
}

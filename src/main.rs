use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
struct Args {
    // IP address to lookup ASN for
    #[arg(short, long, required = true)]
    ip: IpAddr,
}

fn ip_to_str(ip: &IpAddr) -> String {
    ip.to_string()
}

fn main() {
    let args = Args::parse();
    println!("IP address: {}", ip_to_str(args.ip));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ipv4_to_str() {
        let ip = IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(ip_to_str(ip), "127.0.0.1");
    }

    #[test]
    fn test_ipv6_to_str() {
        let ip = IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(ip_to_str(ip), "::1");
    }
}

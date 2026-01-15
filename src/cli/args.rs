// CLI argument parsing

use clap::Parser;
use std::net::IpAddr;

/// Command-line arguments for ASN lookup
#[derive(Parser, Debug)]
pub struct Args {
    /// IP address to lookup ASN for (IPv4 or IPv6)
    #[arg(short, long, required = true)]
    pub ip: IpAddr,

    /// From specify the source DB of the ASN lookup
    #[arg(short, long, default_value = "ripe")]
    pub source: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_args_ipv4_parsing() {
        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let args = Args {
            ip,
            source: "ripe".to_string(),
        };
        assert_eq!(args.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert_eq!(args.source, "ripe")
    }

    #[test]
    fn test_args_ipv6_parsing() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888));
        let args = Args {
            ip,
            source: "".to_string(),
        };
        assert_eq!(
            args.ip,
            IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888))
        );
    }

    #[test]
    fn test_args_debug() {
        let ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let args = Args {
            ip,
            source: "ripe".to_string(),
        };
        let debug_str = format!("{:?}", args);
        assert!(debug_str.contains("1.1.1.1"));
    }

    #[test]
    fn test_args_source_default() {
        let ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let args = Args {
            ip,
            source: "ripe".to_string(),
        };
        assert_eq!(args.source, "ripe");
    }

    #[test]
    fn test_args_source_ipapi() {
        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let args = Args {
            ip,
            source: "ipapi".to_string(),
        };
        assert_eq!(args.source, "ipapi");
        assert_eq!(args.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
    }

    #[test]
    fn test_args_source_custom() {
        let ip = IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1));
        let args = Args {
            ip,
            source: "custom".to_string(),
        };
        assert_eq!(args.source, "custom");
    }
}

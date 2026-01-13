// CLI argument parsing

use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
pub struct Args {
    // IP address to lookup ASN for
    #[arg(short, long, required = true)]
    pub ip: IpAddr,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_args_ipv4_parsing() {
        let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let args = Args { ip };
        assert_eq!(args.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
    }

    #[test]
    fn test_args_ipv6_parsing() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888));
        let args = Args { ip };
        assert_eq!(
            args.ip,
            IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888))
        );
    }

    #[test]
    fn test_args_debug() {
        let ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let args = Args { ip };
        let debug_str = format!("{:?}", args);
        assert!(debug_str.contains("1.1.1.1"));
    }
}

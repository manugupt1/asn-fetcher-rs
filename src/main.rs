use asn_fetcher::asn::{Asn, IPApi, Ripe};
use asn_fetcher::cli::Args;
use clap::Parser;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if env::var_os("ASN_FETCHER_SKIP_LOOKUP").is_some() {
        println!("IP address: {}", args.ip);
        return Ok(());
    }

    let asn_fetcher: Box<dyn Asn> = match args.source.as_str() {
        "ipapi" => Box::new(IPApi::new()?),
        _ => Box::new(Ripe::new()?),
    };
    let asns = asn_fetcher.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

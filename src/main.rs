use asn_fetcher::asn::{Asn, IPApi, Ripe, TeamCymruWhois};
use asn_fetcher::cli::Args;
use clap::Parser;

/// Creates the appropriate ASN fetcher based on the source string
fn create_asn_fetcher(source: &str) -> Result<Box<dyn Asn>, Box<dyn std::error::Error>> {
    match source {
        "ipapi" => Ok(Box::new(IPApi::new()?)),
        "cymru-whois" => Ok(Box::new(TeamCymruWhois)),
        _ => Ok(Box::new(Ripe::new()?)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let asn_fetcher = create_asn_fetcher(&args.source)?;
    let asns = asn_fetcher.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

use asn_fetcher::asn::{Asn, Ripe};
use asn_fetcher::cli::Args;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let r = Ripe::new()?;
    let asns = r.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

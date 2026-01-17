use asn_fetcher::asn::{Asn, IPApi, Ripe};
use asn_fetcher::cli::Args;
use clap::Parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let asn_fetcher: Box<dyn Asn> = match args.source.as_str() {
        "ipapi" => Box::new(IPApi::new()?),
        _ => Box::new(Ripe::new()?),
    };
    let asns = asn_fetcher.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

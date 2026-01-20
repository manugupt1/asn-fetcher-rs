use asn_fetcher::asn::{Asn, IPApi, Ripe, TeamCymruWhois};
use asn_fetcher::cli::Args;
use clap::Parser;

/// Creates the appropriate ASN fetcher based on the source string
fn create_asn_fetcher(source: &str) -> Result<Box<dyn Asn>, Box<dyn std::error::Error>> {
    let (provider, provider_name) = match source {
        "ipapi" => (Box::new(IPApi::new()?) as Box<dyn Asn>, "ipapi"),
        "cymru-whois" => (Box::new(TeamCymruWhois) as Box<dyn Asn>, "cymru-whois"),
        "ripe" => (Box::new(Ripe::new()?) as Box<dyn Asn>, "ripe"),
        _ => {
            eprintln!(
                "Unknown provider '{}', falling back to default provider: ripe",
                source
            );
            (Box::new(Ripe::new()?) as Box<dyn Asn>, "ripe")
        }
    };

    eprintln!("Using provider: {}", provider_name);
    Ok(provider)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let asn_fetcher = create_asn_fetcher(&args.source)?;
    let asns = asn_fetcher.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

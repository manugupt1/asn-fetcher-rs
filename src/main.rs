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

fn main() {
    let args = Args::parse();

    let asn_fetcher = match create_asn_fetcher(&args.source) {
        Ok(fetcher) => fetcher,
        Err(e) => {
            eprintln!("Error: Failed to create ASN fetcher: {}", e);
            std::process::exit(1);
        }
    };

    let asns = match asn_fetcher.lookup_asn(args.ip) {
        Ok(asns) => asns,
        Err(e) => {
            eprintln!("Error: Failed to lookup ASN for {}: {}", args.ip, e);
            std::process::exit(1);
        }
    };

    asns.iter().for_each(|asn| println!("{:?}", asn));
}

use asn_parser::asn::{Asn, Rite};
use asn_parser::cli::Args;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let r = Rite::new()?;
    let asns = r.lookup_asn(args.ip)?;
    asns.iter().for_each(|asn| println!("{:?}", asn));
    Ok(())
}

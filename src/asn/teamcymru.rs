use super::Asn;
use crate::asn::client::format_provider_error;
use std::{
    io::{BufRead, Error, ErrorKind},
    process::{Command, Stdio},
};

use crate::asn::AsnInfo;

/// Looks up from Team Cymru's whois server
pub struct TeamCymruWhois;

impl TeamCymruWhois {
    const CMD: &'static str = "whois";
    const SERVER: &'static str = "whois.cymru.com";

    /// Parses a line of TeamCymruWhois output into an AsnInfo struct.
    /// Example output (first line is a header that is skipped, second is a data line):
    /// AS      | IP               | AS Name
    /// 3561    | 216.90.108.31    | CENTURYLINK-LEGACY-SAVVIS, US
    fn parse_asn_info(line: String) -> Option<AsnInfo> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return None;
        }
        let asn = parts[0].trim().to_string();
        let holder = parts[2].trim().to_string();
        Some(AsnInfo { asn, holder })
    }
}

impl Default for TeamCymruWhois {
    fn default() -> Self {
        Self
    }
}

impl Asn for TeamCymruWhois {
    fn lookup_asn(&self, ip: std::net::IpAddr) -> Result<Vec<AsnInfo>, Error> {
        let output = match Command::new(Self::CMD)
            .args(["-h", Self::SERVER, &ip.to_string()])
            .stdout(Stdio::piped())
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format_provider_error(
                            "TeamCymruWhois",
                            &format!("Command failed: {}", stderr.trim()),
                        ),
                    ));
                }
                output
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format_provider_error(
                        "TeamCymruWhois",
                        &format!("Command failed: {}, is whois installed?", e),
                    ),
                ))
            }
        };

        let mut lines = output.stdout.lines();
        lines.next(); // skip header line

        let asn_infos: Vec<AsnInfo> = lines
            .map_while(Result::ok)
            .map_while(Self::parse_asn_info)
            .collect();
        Ok(asn_infos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teamcymru_default() {
        // Verify that default construction works
        let whois = TeamCymruWhois;
        // Ensure the instance is usable (no panic during construction)
        let _ = whois;

        // Verify the associated constants remain as expected
        assert_eq!(TeamCymruWhois::CMD, "whois");
        assert_eq!(TeamCymruWhois::SERVER, "whois.cymru.com");
    }

    #[test]
    fn test_parse_asn_info_valid() {
        let line = "3561    | 216.90.108.31    | CENTURYLINK-LEGACY-SAVVIS, US".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);

        assert!(result.is_some());
        let asn_info = result.unwrap();
        assert_eq!(asn_info.asn, "3561");
        assert_eq!(asn_info.holder, "CENTURYLINK-LEGACY-SAVVIS, US");
    }

    #[test]
    fn test_parse_asn_info_with_spaces() {
        let line = "  15169  |  8.8.8.8  |  GOOGLE, US  ".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);

        assert!(result.is_some());
        let asn_info = result.unwrap();
        assert_eq!(asn_info.asn, "15169");
        assert_eq!(asn_info.holder, "GOOGLE, US");
    }

    #[test]
    fn test_parse_asn_info_invalid_format() {
        let line = "invalid line without pipes".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_asn_info_too_few_parts() {
        let line = "3561 | 216.90.108.31".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_asn_info_empty_string() {
        let line = "".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_asn_info_with_special_characters() {
        let line = "64512 | 192.168.1.1 | TEST-ASN (Example), US/CA".to_string();
        let result = TeamCymruWhois::parse_asn_info(line);

        assert!(result.is_some());
        let asn_info = result.unwrap();
        assert_eq!(asn_info.asn, "64512");
        assert_eq!(asn_info.holder, "TEST-ASN (Example), US/CA");
    }
}

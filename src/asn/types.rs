// ASN information types

#[derive(Debug, PartialEq)]
pub struct AsnInfo {
    pub asn: String,
    pub holder: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asn_info_creation() {
        let info = AsnInfo {
            asn: "AS15169".to_string(),
            holder: "Google LLC".to_string(),
        };
        assert_eq!(info.asn, "AS15169");
        assert_eq!(info.holder, "Google LLC");
    }

    #[test]
    fn test_asn_info_debug() {
        let info = AsnInfo {
            asn: "AS15169".to_string(),
            holder: "Google LLC".to_string(),
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("AS15169"));
        assert!(debug_str.contains("Google LLC"));
    }
}

// ASN information types

/// Information about an Autonomous System Number (ASN)
///
/// Contains the ASN identifier and the organization that holds it
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsnInfo {
    /// The ASN identifier (e.g., "AS15169")
    pub asn: String,
    /// The organization or entity that holds this ASN
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

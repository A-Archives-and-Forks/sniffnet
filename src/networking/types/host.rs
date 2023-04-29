use crate::networking::types::asn::Asn;

/// Struct to represent a network host
#[derive(Default, PartialEq, Eq, Hash)]
pub struct Host {
    /// Hostname (domain). Obtained from the reverse DNS.
    pub domain: String,
    /// Autonomous System which operates the host
    pub asn: Asn,
    /// Country
    pub country: String,
}

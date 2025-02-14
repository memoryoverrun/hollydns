use std::fmt::Display;

use common_macros::EnumConversions;

#[derive(Debug, PartialEq, EnumConversions)]
#[repr(u16)]
pub enum Type {
    A = 1,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    RP,
    AFSDB,
    X25,
    ISDN,
    RT,
    NSAP,
    #[allow(non_camel_case_types)]
    NSAP_PTR,
    SIG,
    KEY,
    PX,
    GPOS,
    AAAA,
    LOC,
    NXT,
    EID,
    NIMLOC,
    SRV,
    ATMA,
    NAPTR,
    KX,
    CERT,
    A6,
    DNAME,
    SINK,
    OPT,
    APL,
    DS,
    SSHFP,
    IPSECKEY,
    RRSIG,
    NSEC,
    DNSKEY,
    DHCID,
    NSEC3,
    NSEC3PARAM,
    TLSA,
    SMIMEA,
    HIP,
    NINFO,
    RKEY,
    TALINK,
    CDS,
    CDNSKEY,
    OPENPGPKEY,
    CSYNC,
    ZONEMD,
    SVCB,
    HTTPS,
    ANY = 255,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_enum_conversions() {
        assert_eq!(Type::A.to_number(), 1);
        assert_eq!(Type::AAAA.to_number(), 28);
        assert_eq!(Type::CNAME.to_number(), 5);

        assert_eq!(Type::from(1), Type::A);
        assert_eq!(Type::from(28), Type::AAAA);
        assert_eq!(Type::from(5), Type::CNAME);

        assert_eq!(Type::A.to_string(), "A");
        assert_eq!(Type::AAAA.to_string(), "AAAA");
        assert_eq!(Type::CNAME.to_string(), "CNAME");

        assert_eq!(Type::from_str("A").unwrap(), Type::A);
        assert_eq!(Type::from_str("AAAA").unwrap(), Type::AAAA);
        assert_eq!(Type::from_str("CNAME").unwrap(), Type::CNAME);
    }
}

use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug, Eq)]
pub(crate) struct DnsError<'a>(i32, &'a str);

impl<'a> Display for DnsError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl<'a> PartialEq for DnsError<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Error for DnsError<'a> {}

pub(crate) mod dns_error {
    use super::DnsError;

    // header errors
    pub const DNS_ERROR_HEADER_TOO_SHORT: DnsError = DnsError(100001, "Header is too short");
    pub const DNS_ERROR_INVALID_DATA: DnsError = DnsError(100002, "Invalid data");
    pub const DNS_ERROR_INVALID_HEADER: DnsError = DnsError(100003, "Invalid header");
    pub const DNS_ERROR_INVALID_QUERY: DnsError = DnsError(100004, "Invalid query");
    pub const DNS_ERROR_INVALID_RESPONSE: DnsError = DnsError(100005, "Invalid response");

    // label errors
    pub const DNS_ERROR_LABEL_TOO_LONG: DnsError = DnsError(100006, "Label is too long");
    pub const DNS_ERROR_LABEL_LENGTH_MISMATCH: DnsError = DnsError(100007, "Label length mismatch");
    pub const DNS_ERROR_INVALID_CHARACTER: DnsError = DnsError(100008, "Invalid character");

    // dname errors
    pub const DNS_ERROR_INVALID_LABEL: DnsError = DnsError(100009, "Invalid label");
    pub const DNS_ERROR_INVALID_DNAME: DnsError = DnsError(100010, "Invalid dname");
    pub const DNS_ERROR_DNAME_TOO_LONG: DnsError = DnsError(100011, "Dname is too long");
}

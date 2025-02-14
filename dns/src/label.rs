use std::fmt::Display;

use crate::{errors::{dns_error, DnsError}, packet::Packet};

#[derive(Debug, Eq, Clone)]
pub struct Label<'a> {
    length: u8,
    values: &'a [u8],
}

impl<'a> Label<'a> {
    pub fn new(values: &'a [u8]) -> Result<Self, DnsError<'static>> {
        let length = values.len() as u8;
        let label = Self { length, values };
        label.check()?;
        Ok(label)
    }

    pub fn is_root(&self) -> bool {
        self.length == 0
    }

    pub fn is_null(&self) -> bool {
        self.is_root()
    }

    pub fn check(&self) -> Result<(), DnsError<'static>> {
        if self.values.len() > 63 {
            return Err(dns_error::DNS_ERROR_LABEL_TOO_LONG);
        }
        if self.values.len() != self.length as usize {
            return Err(dns_error::DNS_ERROR_LABEL_LENGTH_MISMATCH);
        }
        for c in self.values {
            if !Self::is_valid_char(*c) {
                return Err(dns_error::DNS_ERROR_INVALID_CHARACTER);
            }
        }
        Ok(())
    }

    fn is_valid_char(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'-' || c == b'_'
    }

    pub fn pack(&self, packet: &mut Packet) {
        packet.write_u8(self.length);
        packet.write_bytes(self.values);
    }
}

impl<'a> Display for Label<'a> {    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.values))
    }
}

impl<'a> PartialEq for Label<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }
        for i in 0..self.length {
            let mut a = self.values[i as usize];
            if a.is_ascii_uppercase() {
                a = a.to_ascii_lowercase()
            }
            let mut b = other.values[i as usize];
            if b.is_ascii_uppercase() {
                b = b.to_ascii_lowercase()
            }
            if a != b {
                return false;
            }
        }
        true
    }
}

pub const ROOT_LABEL: Label<'static> = Label {
    length: 0,
    values: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_char() {
        assert!(Label::is_valid_char(b'a'));
        assert!(Label::is_valid_char(b'A'));
        assert!(Label::is_valid_char(b'0'));
        assert!(Label::is_valid_char(b'9'));
        assert!(Label::is_valid_char(b'-'));
        assert!(Label::is_valid_char(b'_'));
        assert!(!Label::is_valid_char(b' '));
        assert!(!Label::is_valid_char(b'&'));
        assert!(!Label::is_valid_char(b'*'));
        assert!(!Label::is_valid_char(b'#'));
        assert!(!Label::is_valid_char(b'$'));
        assert!(!Label::is_valid_char(b'%'));
        assert!(!Label::is_valid_char(b'^'));
        assert!(!Label::is_valid_char(b'~'));
        assert!(!Label::is_valid_char(b'`'));
        assert!(!Label::is_valid_char(b'{'));
        assert!(!Label::is_valid_char(b'}'));
        assert!(!Label::is_valid_char(b'|'));
        assert!(!Label::is_valid_char(b'\\'));
        assert!(!Label::is_valid_char(b'"'));
        assert!(!Label::is_valid_char(b'\''));
        assert!(!Label::is_valid_char(b':'));
        assert!(!Label::is_valid_char(b';'));
        assert!(!Label::is_valid_char(b','));
        assert!(!Label::is_valid_char(b'.'));
        assert!(!Label::is_valid_char(b'/'));
        assert!(!Label::is_valid_char(b'['));
        assert!(!Label::is_valid_char(b']'));
        assert!(!Label::is_valid_char(b'{'));
        assert!(!Label::is_valid_char(b'}'));
        assert!(!Label::is_valid_char(b'('));
        assert!(!Label::is_valid_char(b')'));
        assert!(!Label::is_valid_char(b'*'));
        assert!(!Label::is_valid_char(b'&'));
        assert!(!Label::is_valid_char(b'#'));
        assert!(!Label::is_valid_char(b'$'));
        assert!(!Label::is_valid_char(b'%'));
        assert!(!Label::is_valid_char(b'^'));
        assert!(!Label::is_valid_char(b'~'));
        assert!(!Label::is_valid_char(b'`'));
        assert!(!Label::is_valid_char(b'\\'));
    }

    #[test]
    fn test_label_check() {
        let label = Label::new(b"a");
        assert!(label.is_ok());
        let label = Label::new(b"A");
        assert!(label.is_ok());
        let label = Label::new(b"#");
        assert!(label.err().unwrap() == dns_error::DNS_ERROR_INVALID_CHARACTER);
        let values = vec![b'a'; 256];
        let label = Label::new(&values);
        assert!(label.err().unwrap() == dns_error::DNS_ERROR_LABEL_TOO_LONG);
        let label = Label::new(b"a");
        assert!(label.err().unwrap() == dns_error::DNS_ERROR_LABEL_LENGTH_MISMATCH);
    }
}

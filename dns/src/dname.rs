use crate::{
    errors::{dns_error, DnsError},
    label::Label,
};

pub(crate) struct DName<'a> {
    buffer: &'a [u8],
    labels: Vec<Label<'a>>,
}

impl<'a> DName<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            buffer: name.as_bytes(),
            labels: Vec::new(),
        }
    }

    pub fn parse_from_name(data: &'a [u8]) -> Result<Self, DnsError<'static>> {
        if data.len() > 255 {
            return Err(dns_error::DNS_ERROR_DNAME_TOO_LONG);
        }
        let mut dname = Self {
            buffer: data,
            labels: Vec::new(),
        };
        let mut label_pos = 0;
        let mut seen_root = false;
        for i in 0..dname.buffer.len() {
            if seen_root {
                return Err(dns_error::DNS_ERROR_INVALID_DNAME);
            }
            if dname.buffer[i] != b'.' {
                continue;
            }
            let label = Label::new(i as u8 - label_pos, &dname.buffer[label_pos as usize..i]);
            if let Err(e) = label.check() {
                return Err(e);
            }
            if label.is_root() {
                seen_root = true;
            }
            dname.labels.push(label);
            label_pos = i as u8 + 1;
        }
        if label_pos != dname.buffer.len() as u8 {
            return Err(dns_error::DNS_ERROR_INVALID_DNAME);
        }
        if dname.labels.len() == 0 {
            return Err(dns_error::DNS_ERROR_INVALID_DNAME);
        }
        if !dname.labels.last().unwrap().is_root() {
            return Err(dns_error::DNS_ERROR_INVALID_DNAME);
        }
        Ok(dname)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dname_parse_from_name() {
        let dname = DName::new("www.example.com");
        assert_eq!(dname.labels.len(), 3);
        assert_eq!(dname.labels[0].to_string(), "www");
        assert_eq!(dname.labels[1].to_string(), "example");
        assert_eq!(dname.labels[2].to_string(), "com");
    }
}

use std::fmt::Display;

use crate::{
    errors::{dns_error, DnsError},
    label::Label, packet::Packet,
};

#[derive(Debug)]
pub struct DName<'a> {
    buffer: &'a [u8],
    labels: Vec<Label<'a>>,
}

impl<'a> DName<'a> {
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
            let res = Label::new(&dname.buffer[label_pos as usize..i]);
            if let Err(e) = res {
                return Err(e);
            }
            let label = res.unwrap();
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

    pub fn parse_from_packet(packet: &'a mut Packet) -> Result<Self, DnsError<'static>> {
        let mut dname = Self {
            buffer: packet.as_ref(),
            labels: Vec::new(),
        };
        let mut size = 0;
        let mut offset = packet.rpos();
        let mut c0: u8 = 0;
        while size < 256 {
            c0 = dname.buffer[offset];
            let pointer_flag = c0 & 0xc0;
            if pointer_flag == 0 {
                let len = c0 as usize;
                if offset + 1 + len > packet.as_ref().len() {
                    return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
                }
                let label = Label::new(&dname.buffer[offset + 1..offset + 1 + len])?;
                dname.labels.push(label);
                if offset == packet.rpos() {
                    offset += len + 1;
                    packet.set_rpos(offset);
                }
                if label.is_root() {
                    break;
                }
            } else if pointer_flag == 0xc0 {
                let c1 = packet.peek_u8(offset + 1)?;
                let pointer_offset = u16::from_ne_bytes([c0 & 0x3f, c1]);
                if pointer_offset as usize >= offset {
                    return Err(dns_error::DNS_ERROR_INVALID_POINTER_OFFSET);
                }
                offset = pointer_offset as usize;
            }
        }
        Ok(dname)
    }

    pub fn is_subdname(&self, dname: &Self) -> bool {
        if self.labels.len() >= dname.labels.len() {
            return false;
        }
        for i in (0..dname.labels.len()-1).rev() {
            if self.labels[i] != dname.labels[i] {
                return false;
            }
        }
        true
    }
}

impl<'a> Display for DName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.buffer))
    }
}

impl<'a> PartialEq for DName<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.labels.len() != other.labels.len() {
            return false;
        }
        for i in 0..self.labels.len() {
            if self.labels[i] != other.labels[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dname_parse_from_name() {
        let dname = DName::parse_from_name(b"www.example.com").unwrap();
        assert_eq!(dname.labels.len(), 3);
        assert_eq!(dname.labels[0].to_string(), "www");
        assert_eq!(dname.labels[1].to_string(), "example");
        assert_eq!(dname.labels[2].to_string(), "com");
    }
}

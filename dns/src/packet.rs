use std::borrow::Cow;

use crate::errors::{dns_error, DnsError};

#[derive(Eq, PartialEq)]
pub struct Packet {
    buffer: Vec<u8>,
    wpos: usize,
    rpos: usize,
}

impl Packet {
    pub fn new(capacity: usize) -> Self {
        Self { buffer: Vec::with_capacity(capacity), wpos: 0, rpos: 0 }
    }

    pub fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self { buffer: bytes.into_owned(), wpos: 0, rpos: 0 }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), DnsError> {
        if self.wpos + 1 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
        }
        self.buffer[self.wpos] = value;
        self.wpos += 1;
        Ok(())
    }

    pub fn read_u8(&mut self) -> Result<u8, DnsError> {
        if self.rpos >= self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NO_MORE_DATA);
        }
        let value = self.buffer[self.rpos];
        self.rpos += 1;
        Ok(value)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), DnsError> {
        if self.wpos + 2 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
        }
        #[cfg(target_endian = "little")]
        {
            self.buffer[self.wpos] = value as u8;
            self.buffer[self.wpos + 1] = (value >> 8) as u8;
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer[self.wpos] = (value >> 8) as u8;
            self.buffer[self.wpos + 1] = value as u8;
        }
        self.wpos += 2;
        Ok(())
    }

    pub fn read_u16(&mut self) -> Result<u16, DnsError> {
        if self.rpos + 2 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NO_MORE_DATA);
        }
        let value = u16::from_ne_bytes([self.buffer[self.rpos], self.buffer[self.rpos + 1]]);
        self.rpos += 2;
        Ok(value)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), DnsError> {
        if self.wpos + 4 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
        }
        #[cfg(target_endian = "little")]
        {
            self.buffer[self.wpos] = value as u8;
            self.buffer[self.wpos + 1] = (value >> 8) as u8;
            self.buffer[self.wpos + 2] = (value >> 16) as u8;
            self.buffer[self.wpos + 3] = (value >> 24) as u8;
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer[self.wpos] = (value >> 24) as u8;
            self.buffer[self.wpos + 1] = (value >> 16) as u8;
            self.buffer[self.wpos + 2] = (value >> 8) as u8;
            self.buffer[self.wpos + 3] = value as u8;
        }
        self.wpos += 4;
        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32, DnsError> {
        if self.rpos + 4 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NO_MORE_DATA);
        }
        let bytes = [self.buffer[self.rpos], self.buffer[self.rpos + 1], self.buffer[self.rpos + 2], self.buffer[self.rpos + 3]];
        let value = u32::from_ne_bytes(bytes);
        self.rpos += 4;
        Ok(value)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), DnsError> {
        if self.wpos + 8 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
        }
        #[cfg(target_endian = "little")]
        {
            self.buffer[self.wpos] = value as u8;
            self.buffer[self.wpos + 1] = (value >> 8) as u8;
            self.buffer[self.wpos + 2] = (value >> 16) as u8;
            self.buffer[self.wpos + 3] = (value >> 24) as u8;
            self.buffer[self.wpos + 4] = (value >> 32) as u8;
            self.buffer[self.wpos + 5] = (value >> 40) as u8;
            self.buffer[self.wpos + 6] = (value >> 48) as u8;
            self.buffer[self.wpos + 7] = (value >> 56) as u8;
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer[self.wpos] = (value >> 56) as u8;
            self.buffer[self.wpos + 1] = (value >> 48) as u8;
            self.buffer[self.wpos + 2] = (value >> 40) as u8;
            self.buffer[self.wpos + 3] = (value >> 32) as u8;
            self.buffer[self.wpos + 4] = (value >> 24) as u8;
            self.buffer[self.wpos + 5] = (value >> 16) as u8;
            self.buffer[self.wpos + 6] = (value >> 8) as u8;
            self.buffer[self.wpos + 7] = value as u8;
        }
        self.wpos += 8;
        Ok(())
    }
    
    pub fn read_u64(&mut self) -> Result<u64, DnsError> {
        if self.rpos + 8 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NO_MORE_DATA);
        }
        let bytes = [self.buffer[self.rpos], self.buffer[self.rpos + 1], self.buffer[self.rpos + 2], self.buffer[self.rpos + 3], self.buffer[self.rpos + 4], self.buffer[self.rpos + 5], self.buffer[self.rpos + 6], self.buffer[self.rpos + 7]];
        let value = u64::from_ne_bytes(bytes);
        self.rpos += 8;
        Ok(value)
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> Result<(), DnsError> {
        if self.wpos + value.len() > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NOT_ENOUGH_SPACE);
        }
        self.buffer[self.wpos..self.wpos + value.len()].copy_from_slice(value);
        self.wpos += value.len();
        Ok(())
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, DnsError> {
        if self.rpos + len > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_PACKET_NO_MORE_DATA);
        }
        let result = self.buffer[self.rpos..self.rpos + len].to_vec();
        self.rpos += len;
        Ok(result)
    }

    pub fn read_slice(&mut self, len: usize) -> Result<Vec<u8>, DnsError> {
        if len > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        let result = self.buffer[self.rpos..self.rpos + len].to_vec();
        self.rpos += len;
        Ok(result)
    }

    pub fn peek_u8(&self, pos: usize) -> Result<u8, DnsError> {
        if pos >= self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        Ok(self.buffer[pos])
    }

    pub fn peek_u16(&self, pos: usize) -> Result<u16, DnsError> {
        if pos + 2 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        Ok(u16::from_ne_bytes([self.buffer[pos], self.buffer[pos + 1]]))
    }

    pub fn peek_u32(&self, pos: usize) -> Result<u32, DnsError> {
        if pos + 4 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        Ok(u32::from_ne_bytes([self.buffer[pos], self.buffer[pos + 1], self.buffer[pos + 2], self.buffer[pos + 3]]))
    }

    pub fn peek_u64(&self, pos: usize) -> Result<u64, DnsError> {
        if pos + 8 > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        Ok(u64::from_ne_bytes([self.buffer[pos], self.buffer[pos + 1], self.buffer[pos + 2], self.buffer[pos + 3], self.buffer[pos + 4], self.buffer[pos + 5], self.buffer[pos + 6], self.buffer[pos + 7]]))
    }

    pub fn peek_bytes(&self, pos: usize, len: usize) -> Result<Vec<u8>, DnsError> {
        if pos + len > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        Ok(self.buffer[pos..pos + len].to_vec())
    }

    pub fn rpos(&self) -> usize {
        self.rpos
    }

    pub fn set_rpos(&mut self, pos: usize) {
        self.rpos = pos;
    }

    pub fn wpos(&self) -> usize {
        self.wpos
    }
}

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}

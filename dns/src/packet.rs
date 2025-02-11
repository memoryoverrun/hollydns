use crate::errors::{dns_error, DnsError};

pub(crate) struct Packet {
    buffer: Vec<u8>,
    capacity: usize,
}

impl Packet {
    pub fn new() -> Self {
        Self { buffer: Vec::new(), capacity: 0 }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self { buffer: bytes.to_vec(), capacity: bytes.len() }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
        self.capacity += 1;
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = self.buffer[0];
        self.buffer.remove(0);
        self.capacity -= 1;
        value
    }

    pub fn write_u16(&mut self, value: u16) {
        #[cfg(target_endian = "little")]
        {
            self.buffer.push(value as u8);
            self.buffer.push((value >> 8) as u8);
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer.push((value >> 8) as u8);
            self.buffer.push(value as u8);
        }
        self.capacity += 2;
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes([self.buffer[0], self.buffer[1]]);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.capacity -= 2;
        value
    }

    pub fn write_u32(&mut self, value: u32) {
        #[cfg(target_endian = "little")]
        {
            self.buffer.push(value as u8);
            self.buffer.push((value >> 8) as u8);
            self.buffer.push((value >> 16) as u8);
            self.buffer.push((value >> 24) as u8);
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer.push((value >> 24) as u8);
            self.buffer.push((value >> 16) as u8);
            self.buffer.push((value >> 8) as u8);
            self.buffer.push(value as u8);
        }
        self.capacity += 4;
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes([self.buffer[0], self.buffer[1], self.buffer[2], self.buffer[3]]);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.capacity -= 4;
        value
    }

    pub fn write_u64(&mut self, value: u64) {
        #[cfg(target_endian = "little")]
        {
            self.buffer.push(value as u8);
            self.buffer.push((value >> 8) as u8);
        }
        #[cfg(target_endian = "big")]
        {
            self.buffer.push((value >> 56) as u8);
            self.buffer.push((value >> 48) as u8);
            self.buffer.push((value >> 40) as u8);
            self.buffer.push((value >> 32) as u8);
            self.buffer.push((value >> 24) as u8);
            self.buffer.push((value >> 16) as u8);
            self.buffer.push((value >> 8) as u8);
            self.buffer.push(value as u8);
        }
        self.capacity += 8;
    }
    
    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes([self.buffer[0], self.buffer[1], self.buffer[2], self.buffer[3], self.buffer[4], self.buffer[5], self.buffer[6], self.buffer[7]]);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.buffer.remove(0);
        self.capacity -= 8;
        value
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        self.buffer.extend_from_slice(value);
        self.capacity += value.len();
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        self.capacity = 0;
        std::mem::take(&mut self.buffer)
    }

    pub fn read_slice(&mut self, len: usize) -> Result<Vec<u8>, DnsError<'static>> {
        if len > self.buffer.len() {
            return Err(dns_error::DNS_ERROR_INVALID_DATA);
        }
        let result = self.buffer[..len].to_vec();
        self.buffer = self.buffer[len..].to_vec();
        self.capacity -= len;
        Ok(result)
    }
}

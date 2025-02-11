use std::mem;

use bitflags::bitflags;
use common_macros::EnumConversions;

use crate::{errors::{dns_error, DnsError}, packet, ID};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    struct Flags: u16 {
        const FLAG_QR = 0b0000_0000_0000_0001;
        const FLAG_OPCODE = 0b0000_0000_0001_1110;
        const FLAG_AA = 0b0000_0000_0010_0000;
        const FLAG_TC = 0b0000_0000_0100_0000;
        const FLAG_RD = 0b0000_0000_1000_0000;
        const FLAG_RA = 0b0000_0001_0000_0000;
        const FLAG_Z = 0b0000_1110_0000_0000;
        const FLAG_RCODE = 0b1111_0000_0000_0000;
    }
}

impl Flags {
    pub fn new(ne: u16) -> Self {
        Self::from_bits_truncate(ne)
    }

    pub fn is_query(&self) -> bool {
        !self.contains(Flags::FLAG_QR)
    }

    pub fn set_query(&mut self) {
        self.set(Flags::FLAG_QR, false);
    }

    pub fn is_response(&self) -> bool {
        self.contains(Flags::FLAG_QR)
    }

    pub fn set_response(&mut self) {
        self.set(Flags::FLAG_QR, true);
    }

    pub fn op_code(&self) -> OpCode {
        let bits = self.intersection(Flags::FLAG_OPCODE).bits();
        OpCode::from(bits >> 1)
    }

    pub fn set_op_code(&mut self, op_code: OpCode) {
        let bits = op_code.to_number() << 1;
        self.remove(Flags::FLAG_OPCODE);
        self.insert(Flags::from_bits_truncate(bits));
    }

    pub fn is_authoritative(&self) -> bool {
        self.contains(Flags::FLAG_AA)
    }

    pub fn set_authoritative(&mut self, authoritative: bool) {
        self.set(Flags::FLAG_AA, authoritative);
    }

    pub fn is_truncated(&self) -> bool {
        self.contains(Flags::FLAG_TC)
    }

    pub fn set_truncated(&mut self, truncated: bool) {
        self.set(Flags::FLAG_TC, truncated);
    }

    pub fn is_recursion_desired(&self) -> bool {
        self.contains(Flags::FLAG_RD)
    }

    pub fn set_recursion_desired(&mut self, recursion_desired: bool) {
        self.set(Flags::FLAG_RD, recursion_desired);
    }

    pub fn is_recursion_available(&self) -> bool {
        self.contains(Flags::FLAG_RA)
    }

    pub fn set_recursion_available(&mut self, recursion_available: bool) {
        self.set(Flags::FLAG_RA, recursion_available);
    }

    pub fn r_code(&self) -> RCode {
        let bits = self.intersection(Flags::FLAG_RCODE).bits();
        RCode::from(bits >> 12)
    }

    pub fn set_r_code(&mut self, r_code: RCode) {
        let bits = r_code.to_number() << 12;
        self.remove(Flags::FLAG_RCODE);
        self.insert(Flags::from_bits_truncate(bits));
    }
}

#[derive(Debug, EnumConversions)]
#[repr(u16)]
enum OpCode {
    Query,
    IQuery,
    Status,
    Notify,
    Update,
}

#[derive(Debug, EnumConversions)]
#[repr(u16)]
enum RCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

#[repr(packed)]
pub(crate) struct Header {
    id: ID,
    flags: Flags,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
}

impl Header {
    pub fn new() -> Self {
        Self { id: 0, flags: Flags::empty(), qd_count: 0, an_count: 0, ns_count: 0, ar_count: 0 }
    }

    pub fn parse(data: &[u8]) -> Result<Self, DnsError> {
        if data.len() < mem::size_of::<Self>() {
            return Err(dns_error::DNS_ERROR_HEADER_TOO_SHORT);
        }
        let mut header = Self::new();
        header.id = u16::from_ne_bytes([data[0], data[1]]);
        header.flags = Flags::new(u16::from_ne_bytes([data[2], data[3]]));
        header.qd_count = u16::from_be_bytes([data[4], data[5]]);
        header.an_count = u16::from_be_bytes([data[6], data[7]]);
        header.ns_count = u16::from_be_bytes([data[8], data[9]]);
        header.ar_count = u16::from_be_bytes([data[10], data[11]]);
        Ok(header)
    }

    pub fn to_bytes(&self) -> [u8; mem::size_of::<Self>()] {
        let mut bytes = [0; mem::size_of::<Self>()];
        let flags = self.flags;
        #[cfg(target_endian = "little")]
        {
            bytes[0] = self.id as u8;
            bytes[1] = (self.id >> 8) as u8;
            bytes[2] = flags.bits() as u8;
            bytes[3] = (flags.bits() >> 8) as u8;
            bytes[4] = self.qd_count as u8;
            bytes[5] = (self.qd_count >> 8) as u8;
            bytes[6] = self.an_count as u8;
            bytes[7] = (self.an_count >> 8) as u8;
            bytes[8] = self.ns_count as u8;
            bytes[9] = (self.ns_count >> 8) as u8;
            bytes[10] = self.ar_count as u8;
            bytes[11] = (self.ar_count >> 8) as u8;
        }
        #[cfg(target_endian = "big")]
        {
            bytes[0] = (self.id >> 8) as u8;
            bytes[1] = self.id as u8;
            bytes[2] = (flags >> 8) as u8;
            bytes[3] = flags as u8;
            bytes[4] = (self.qd_count >> 8) as u8;
            bytes[5] = self.qd_count as u8;
            bytes[6] = (self.an_count >> 8) as u8;
            bytes[7] = self.an_count as u8;
            bytes[8] = (self.ns_count >> 8) as u8;
            bytes[9] = self.ns_count as u8;
            bytes[10] = (self.ar_count >> 8) as u8;
            bytes[11] = self.ar_count as u8;
        }
        bytes
    }

    pub fn pack(&self, packet: &mut packet::Packet) {
        packet.pack_u16(self.id);
        let flags = self.flags;
        packet.pack_u16(flags.bits());
        packet.pack_u16(self.qd_count);
        packet.pack_u16(self.an_count);
        packet.pack_u16(self.ns_count);
        packet.pack_u16(self.ar_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header() {
        println!("Alignment of Header: {}", mem::align_of::<Header>())
    }
    
}

use crate::packet::Packet;
use crate::rdatas::RData;
use std::fmt;
use std::fmt::Display;
use std::net::Ipv4Addr;

pub struct A {
    pub address: Ipv4Addr,
}

impl RData for A {
    fn pack(&self, packet: &mut Packet) {
        packet.write_u32(self.address.into());
    }

    fn parse_from_packet(packet: &mut Packet) -> Result<Self, crate::errors::DnsError> {
        Ok(A { address: Ipv4Addr::from(packet.read_u32()?) })
    }
}

impl Display for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut packet = Packet::from_bytes(&[0x58, 0xdd, 0x51, 0xc0]);
        let rdata_a = A::parse_from_packet(&mut packet).unwrap();
        assert_eq!(rdata_a.to_string(), "88.221.81.192");
    }
}

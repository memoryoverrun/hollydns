include!("rdatas_generated.rs");

use std::fmt::Display;
use crate::packet::Packet;
use crate::errors::DnsError;

pub trait RData: Display {
    fn pack(&self, packet: &mut Packet);
    fn parse_from_packet(packet: &mut Packet) -> Result<Self, DnsError> where Self: Sized;
}

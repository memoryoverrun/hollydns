use std::fmt::Display;

use crate::{class::Class, dname::DName, rdatas::RData, r#type::Type};

#[derive(Debug)]
pub struct Record<'a, T> {
    pub dname: DName<'a>,
    pub rtype: Type,
    pub rclass: Class,
    pub ttl: u32,
    pub rdata: T,
}

impl<'a, T> Record<'a, T>
where
    T: RData,
{
    pub fn new(dname: DName<'a>, rtype: Type, rclass: Class, ttl: u32, rdata: T) -> Self {
        Self { dname, rtype, rclass, ttl, rdata }
    }
}

impl<'a, T> Display for Record<'a, T>
where
    T: Display + RData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {}", self.dname, self.rtype, self.rclass, self.ttl, self.rdata)
    }
}

use crate::{class::Class, dname::DName, rdatas::RData, record::Record, r#type::Type};
use std::fmt::Display;

#[derive(Debug)]
pub struct RecordSet<'a, T> {
    pub dname: DName<'a>,
    pub rtype: Type,
    pub rclass: Class,
    pub ttl: u32,
    pub records: Vec<Record<'a, T>>,
}

impl<'a, T> RecordSet<'a, T> {
    pub fn new(dname: DName<'a>, rtype: Type, rclass: Class, ttl: u32, records: Vec<Record<'a, T>>) -> Self {
        Self { dname, rtype, rclass, ttl, records }
    }
}

impl<'a, T> Display for RecordSet<'a, T>
where
    T: Display + RData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.dname, self.rtype, self.rclass, self.ttl)?;
        for record in &self.records {
            write!(f, " {}", record)?;
        }
        Ok(())
    }
}

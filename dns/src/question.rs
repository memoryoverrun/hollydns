use std::fmt::Display;

use crate::{class::Class, dname::DName, r#type::Type};

pub struct Question<'a> {
    name: DName<'a>,
    qtype: Type,
    qclass: Class,
}

impl<'a> Question<'a> {
    pub fn new(name: DName<'a>, qtype: Type, qclass: Class) -> Self {
        Self { name, qtype, qclass }
    }
}

impl<'a> Display for Question<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.qtype, self.qclass)
    }
}


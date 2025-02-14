use std::fmt::Display;

use common_macros::EnumConversions;


#[derive(Debug, PartialEq, EnumConversions)]
#[repr(u16)]
pub enum Class {
    IN = 1,
    CS,
    CH,
    HS,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        assert_eq!(Class::IN.to_number(), 1);
        assert_eq!(Class::CS.to_number(), 2);
        assert_eq!(Class::CH.to_number(), 3);
        assert_eq!(Class::HS.to_number(), 4);
    }
}

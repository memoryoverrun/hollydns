use common_macros::EnumConversions;

#[derive(Debug, PartialEq, EnumConversions)]
pub enum Status {
    Active = 1,
    Inactive,  // 将自动为2
    Pending = 5,
    Deleted    // 将自动为6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(Status::Active.to_number(), 1);
        assert_eq!(Status::Inactive.to_number(), 2);
        assert_eq!(Status::Pending.to_number(), 5);
        assert_eq!(Status::Deleted.to_number(), 6);
    }
}

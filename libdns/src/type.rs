#[repr(u16)]
enum Type {
    A,
    AAAA,
    CNAME,
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Type::A,
        }
    }
}

impl From<Type> for u16 {
    fn from(value: Type) -> Self {
        value.into()
    }
}

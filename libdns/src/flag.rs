enum Flag {
    QR,
    AA,
    TC,
}

impl Flag {
    fn to_u16(&self) -> u16 {
        match self {
            Flag::QR => 0x8000,
            Flag::AA => 0x0400,
            Flag::TC => 0x0200,
        }
    }
}

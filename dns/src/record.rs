pub struct Record {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

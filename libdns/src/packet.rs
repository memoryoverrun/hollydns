// DNS packet
pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additional: Vec<ResourceRecord>,
}

pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub questions: u16,
    pub answers: u16,
}

pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional: Vec::new(),
        }
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            id: 0,
            flags: 0,
            questions: 0,
            answers: 0,
        }
    }
}

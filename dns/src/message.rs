use crate::{header::Header, question::Question, rdatas::RData, record_set::RecordSet};

pub struct Message<'a> {
    pub header: Header,
    pub questions: Question<'a>,
    pub answers: Vec<RecordSet<'a, Box<dyn RData>>>,
    pub authorities: Vec<RecordSet<'a, Box<dyn RData>>>,
    pub additional: Vec<RecordSet<'a, Box<dyn RData>>>
}

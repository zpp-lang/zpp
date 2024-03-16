use codespan::{ByteIndex, Span};

pub mod backend;
pub mod frontend;

pub fn create_span(log: logos::Span) -> Span {
    Span::new(
        ByteIndex::from(log.start as u32),
        ByteIndex::from(log.end as u32)
    )
}
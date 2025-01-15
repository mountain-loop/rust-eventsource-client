#![warn(rust_2018_idioms)]

mod error;
mod event_parser;

pub use error::*;
pub use event_parser::Event;
pub use event_parser::EventParser;
pub use event_parser::SSE;

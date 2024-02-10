use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_CAPS_DETECTION: Regex = Regex::new(r"[a-z]{1}[A-Z]{2,} ").unwrap();
    static ref NOT_WORD: Regex = Regex::new(r"[^a-zA-Z]{1,}").unwrap();
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum BufferStatus {
    NothingSpecial,
    WordFinished,
    WrongCapsDetected,
}

pub fn analyse_str_state(buffer: String) -> BufferStatus {
    if WRONG_CAPS_DETECTION.is_match(&buffer) {
        BufferStatus::WrongCapsDetected
    } else if NOT_WORD.is_match(&buffer) {
        BufferStatus::WordFinished
    } else {
        BufferStatus::NothingSpecial
    }
}

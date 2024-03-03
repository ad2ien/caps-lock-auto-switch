use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_CAPS_DETECTION: Regex = Regex::new(r"^[a-z]{1}[A-Z]{2,} ").unwrap();
    static ref NOT_WORD: Regex = Regex::new(r"[^a-zA-Z]{1,}").unwrap();
}

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analyse_str_state() {
        let buffer = String::from("hELLO ");
        assert_eq!(analyse_str_state(buffer), BufferStatus::WrongCapsDetected);

        let buffer = String::from("Hello ");
        assert_eq!(analyse_str_state(buffer), BufferStatus::WordFinished);

        let buffer = String::from("Hello");
        assert_eq!(analyse_str_state(buffer), BufferStatus::NothingSpecial);

        let buffer = String::from("HeLLO ");
        assert_eq!(analyse_str_state(buffer), BufferStatus::WordFinished);
    }
}

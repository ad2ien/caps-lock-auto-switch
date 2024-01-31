use lazy_static::lazy_static;
use regex::Regex;
use std::sync::RwLock;

use notify_rust::Notification;
use rdev::{listen, Event};
use state::InitCell;

static STATE: InitCell<RwLock<CapsLockAutoSwitchState>> = InitCell::new();

lazy_static! {
    static ref WRONG_CAPS_DETECTION: Regex = Regex::new(r"[a-z]{1}[A-Z]{2,} ").unwrap();
    static ref NOT_WORD: Regex = Regex::new(r"[^a-zA-Z]{1,}").unwrap();
}

struct CapsLockAutoSwitchState {
    input: String,
}

enum BufferStatus {
    NothingSpecial,
    WordFinished,
    WrongCapsDetected,
}

fn main() {
    println!("Start Caps-Lock Auto Switch!");

    let state = CapsLockAutoSwitchState {
        input: String::new(),
    };

    STATE.set(RwLock::new(state));

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    match event.name {
        Some(string) => {
            key_pressed(string);
            match analyse_state() {
                BufferStatus::WordFinished => {
                    reset_buffer();
                }
                BufferStatus::WrongCapsDetected => {
                    wrong_case_detected();
                    reset_buffer();
                }
                _ => (),
            }
        }
        None => (),
    }
}

fn key_pressed(string_key: String) {
    let mut w_state = STATE.get().write().unwrap();
    w_state.input.push_str(&string_key);
}


fn analyse_state() -> BufferStatus {

    let state = STATE.get().read().unwrap();
    println!("state: {}", state.input);

    if WRONG_CAPS_DETECTION.is_match(&state.input) {
        BufferStatus::WrongCapsDetected
    } else if NOT_WORD.is_match(&state.input) {
        BufferStatus::WordFinished
    } else {
        BufferStatus::NothingSpecial
    }
}

fn reset_buffer() {
    println!("reset buffer");
    match STATE.get().try_write() {
        Ok(mut mut_state) => mut_state.input.clear(),
        Err(_) => println!("Lock error: reset_buffer"),
    }
}

fn wrong_case_detected() {
    println!("wrong case detected!");

    let state = STATE.get().read().unwrap();

    Notification::new()
        .summary("Wrong case detected!")
        .body(format!("Are you sure about the case of this word : {}", state.input).as_str())
        .icon("dialog-info")
        .timeout(10000)
        .show()
        .unwrap();
}

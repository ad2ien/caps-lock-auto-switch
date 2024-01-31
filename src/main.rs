use regex::Regex;
use std::sync::RwLock;

use rdev::{listen, Event};
use state::InitCell;

static STATE: InitCell<RwLock<CapsLockAutoSwitchState>> = InitCell::new();

const WRONG_CAPS_DETECTION: &str = r"[a-z]{1}[A-Z]{2,}";
const NOT_WORD: &str = r"[^a-zA-Z]{1,}";

struct CapsLockAutoSwitchState {
    input: String,
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
            if analyse_state() {
                reset_buffer();
            }
        }
        None => (),
    }
}

fn key_pressed(string_key: String) {
    println!("key pressed: {} ", string_key);
    let wr_state = STATE.get();
    let mut w_state = wr_state.write().unwrap();
    w_state.input.push_str(&string_key);
}

fn analyse_state() -> bool {
    let state = STATE.get().read().unwrap();
    println!("state: {}", state.input);

    let re: Regex = Regex::new(NOT_WORD).unwrap();
    return re.is_match(&state.input);
}

fn reset_buffer() {
    println!("reset buffer");
    match STATE.get().try_write() {
        Ok(mut mut_state) => mut_state.input.clear(),
        Err(_) => println!("Error: reset_buffer"),
    }
}

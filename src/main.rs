use lazy_static::lazy_static;
use regex::Regex;
use std::{env, path::PathBuf, sync::RwLock, thread, time};

use notify_rust::Notification;
use rdev::{listen, simulate, Event, EventType, Key};
use state::InitCell;
mod config;

static STATE: InitCell<RwLock<CapsLockAutoSwitchState>> = InitCell::new();

lazy_static! {
    static ref WRONG_CAPS_DETECTION: Regex = Regex::new(r"[a-z]{1}[A-Z]{2,} ").unwrap();
    static ref NOT_WORD: Regex = Regex::new(r"[^a-zA-Z]{1,}").unwrap();
    #[derive(Debug)]
    static ref CURRENT_EXE: PathBuf = match env::current_exe() {
             Ok(exe_path) => exe_path,
             Err(_e) => PathBuf::from(""),
         };
    static ref CONFIG: config::Config = match config::get_config() {
        Ok(cfg) => cfg,
        Err(_) => {
            print!("Error loading config, using default");
            config::Config::default()
        }
    };

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
    println!(
        "Current exe: {}",
        match CURRENT_EXE.to_str() {
            Some(s) => s,
            None => "unknown",
        }
    );

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

    correct_caps(state.input.clone());
    do_notification(state);
}

fn do_notification(state: std::sync::RwLockReadGuard<'_, CapsLockAutoSwitchState>) {
    let template_message = ": are you sure about the case of this word?";

    let message = match CURRENT_EXE.to_str() {
        Some(s) => {
            format!(
                "{}{}\nYou can change the settings there : {}/config.yaml",
                state.input, template_message, s
            )
        }
        None => {
            format!("{}{}", template_message, state.input)
        }
    };

    Notification::new()
        .summary("Wrong case detected!")
        .body(message.as_str())
        .icon("dialog-information")
        .timeout(10000)
        .show()
        .unwrap();
}

fn correct_caps(problematic_word: String) {
    println!(
        "correct caps send event. problematic_word: {}",
        problematic_word
    );

    let mut first = true;

    problematic_word.clone().chars().for_each(|_| {
        send(&EventType::KeyPress(Key::Backspace));
        send(&EventType::KeyRelease(Key::Backspace));
    });

    problematic_word.clone().chars().for_each(|c| {
        let key = char_to_key(c);
        if key == Key::Alt {
            println!("error finding key associated to char: {}", c);
            return;
        }
        send(&EventType::KeyPress(key));
        send(&EventType::KeyRelease(key));

        if first {
            send(&EventType::KeyPress(Key::CapsLock));
            send(&EventType::KeyRelease(Key::CapsLock));
            first = false;
        }
    });
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(_simulate_error) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn char_to_key(c: char) -> Key {
    match c {
        'a' => Key::KeyA,
        'b' => Key::KeyB,
        'c' => Key::KeyC,
        'd' => Key::KeyD,
        'e' => Key::KeyE,
        'f' => Key::KeyF,
        'g' => Key::KeyG,
        'h' => Key::KeyH,
        'i' => Key::KeyI,
        'j' => Key::KeyJ,
        'k' => Key::KeyK,
        'l' => Key::KeyL,
        'm' => Key::KeyM,
        'n' => Key::KeyN,
        'o' => Key::KeyO,
        'p' => Key::KeyP,
        'q' => Key::KeyQ,
        'r' => Key::KeyR,
        's' => Key::KeyS,
        't' => Key::KeyT,
        'u' => Key::KeyU,
        'v' => Key::KeyV,
        'w' => Key::KeyW,
        'x' => Key::KeyX,
        'y' => Key::KeyY,
        'z' => Key::KeyZ,
        'A' => Key::KeyA,
        'B' => Key::KeyB,
        'C' => Key::KeyC,
        'D' => Key::KeyD,
        'E' => Key::KeyE,
        'F' => Key::KeyF,
        'G' => Key::KeyG,
        'H' => Key::KeyH,
        'I' => Key::KeyI,
        'J' => Key::KeyJ,
        'K' => Key::KeyK,
        'L' => Key::KeyL,
        'M' => Key::KeyM,
        'N' => Key::KeyN,
        'O' => Key::KeyO,
        'P' => Key::KeyP,
        'Q' => Key::KeyQ,
        'R' => Key::KeyR,
        'S' => Key::KeyS,
        'T' => Key::KeyT,
        'U' => Key::KeyU,
        'V' => Key::KeyV,
        'W' => Key::KeyW,
        'X' => Key::KeyX,
        'Y' => Key::KeyY,
        'Z' => Key::KeyZ,
        _ => Key::Alt,
    }
}

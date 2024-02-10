use lazy_static::lazy_static;
use std::{env, path::PathBuf, sync::RwLock, thread, time};

use notify_rust::Notification;
use rdev::{listen, simulate, Event, EventType, Key};
use buffer_process::{analyse_str_state, BufferStatus};
use state::InitCell;

mod buffer_process;
mod config;

static STATE: InitCell<RwLock<CapsLockAutoSwitchState>> = InitCell::new();

lazy_static! {
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

struct SingleEvent {
    key: Key,
    character: char,
}

struct CapsLockAutoSwitchState {
    input: Vec<SingleEvent>,
}

impl CapsLockAutoSwitchState {
    fn new() -> CapsLockAutoSwitchState {
        CapsLockAutoSwitchState { input: Vec::new() }
    }
    fn to_string(&self) -> String {
        let mut result = String::new();
        self.input.iter().for_each(|event| {
            result.push(event.character);
        });
        result
    }
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

    let state = CapsLockAutoSwitchState::new();
    STATE.set(RwLock::new(state));

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    match event.clone().name {
        Some(_string) => {
            key_pressed(event);
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

fn key_pressed(event: Event) {
    let mut w_state = STATE.get().write().unwrap();
    match event.event_type {
        EventType::KeyPress(key) => {
            let event = SingleEvent {
                key: key,
                character: event.name.unwrap().chars().next().unwrap(),
            };
            w_state.input.push(event);
        }
        EventType::KeyRelease(_) => (),
        _ => (),
    }
}

fn analyse_state() -> BufferStatus {
    let state = STATE.get().read().unwrap();
    println!("state: {}", state.to_string());
    analyse_str_state(state.to_string())
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

    correct_caps(state);
    do_notification();
}

fn do_notification() {
    let state = STATE.get().read().unwrap();
    let template_message = ": are you sure about the case of this word?";

    let message = match CURRENT_EXE.to_str() {
        Some(s) => {
            format!(
                "{}{}\nYou can change the settings there : {}/config.yaml",
                state.to_string(),
                template_message,
                s
            )
        }
        None => {
            format!("{}{}", template_message, state.to_string())
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

fn correct_caps(problematic_state: std::sync::RwLockReadGuard<'_, CapsLockAutoSwitchState>) {
    println!(
        "correct caps send event. problematic_word: {}",
        problematic_state.to_string()
    );

    let mut first = true;

    for _ in 0..problematic_state.input.len() {
        send(&EventType::KeyPress(Key::Backspace));
        send(&EventType::KeyRelease(Key::Backspace));
    }

    for key_tuple in problematic_state.input.iter() {
        let key = key_tuple.key;
        send(&EventType::KeyPress(key));
        send(&EventType::KeyRelease(key));

        if first {
            send(&EventType::KeyPress(Key::CapsLock));
            send(&EventType::KeyRelease(Key::CapsLock));
            first = false;
        }
    }
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

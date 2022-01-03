# grabber
## A simple windows/linux keylogger that pays attention to detail.

### Features

By default, it will not: 
- Log a word if the user typed a space after it.
- Log non-ASCII characters such as emojis.
- Log a key-combination. e.g. ^C

It will also track a few things to make it more useful:
- Where the cursor is on a line to ensure accurate output
- Whether a keyboard shortcut is being typed
- Whether CTRL is being typed (for "power-user" shortcuts such as moving through/deleting words.).

<kbd>Ctrl</kbd> + <kbd>Backspace</kbd> (delete word before cursor) is a great example of something that is handled properly.

Already works on Linux and I am testing it on Windows, it can log to anywhere you like by changing the append_to_log function in `src/lib.rs`. Or ask me and I can do it for you provided it is not too complicated 😅

### Configuration (see `./src/lib.rs`)

- `MODIFIER_KEYS`: a list of keys to count as shortcuts
- `TIMEOUT`: timeout after modifier press
- `HOOK_URL`: a Discord webhook URL, encoded in url-safe base64. (you don't need this if you implement your own logging mechanism in `lib.rs`)
- `GET_SENTENCES`:  Whether to ignore typed characters when a space is encountered. Set this to false to have better luck logging passwords.

### Bugs

If you encounter bugs:

- Leave an issue
- DM me on discord: smiley#5012

## Supported OSes/Environments

Open a PR to edit these.

- Artix Linux (almost stock dwm on xorg)
- Windows 10

### Disclaimer

Only deploy the software on computers you have been given express permission to use this on. 
I will take zero responsibility for misuse of this software.

# passwordgrabber-rs

## it grabs passwords by logging keys

It is a keylogger that ignore anything with a space in it so you only get passwords.

It can keep track of a few things to make it more useful.

- where the cursor is on a line (no more [BACKSPC] [RIGHTARRW])
- whether a keyboard shortcut is being typed
- whether ctrl is being typed.

It works on Linux and I am going to test it on Windows, it logs to a Discord webhook at the moment.

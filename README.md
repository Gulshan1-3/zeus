Raw Mode Terminal in Rust

Overview

This Rust program enables raw mode for terminal input, allowing immediate character processing without requiring Enter. It disables standard input processing features like canonical mode, echo, and signal handling. This setup is useful for building text editors, terminal applications, or interactive command-line tools.

Features

Raw Mode Input Handling: Reads and processes input one character at a time.

Control Character Detection: Displays binary and ASCII representations of control keys.

Screen Clearing: Refreshes the terminal screen after every key press.

Custom Quit Command: Exits when Ctrl + Q is pressed.

Dependencies

Rust

libc (for Unix-based terminal control functions)

Installation

cargo build --release

Usage

Run the executable:

cargo run

Controls

Press any key to see its binary and ASCII representation.

Press Ctrl + Q to exit raw mode and restore the terminal settings.

use std::io::{self, Read};

fn main() {
    let mut buffer = [0; 1]; // Single-byte buffer
    let stdin = io::stdin(); // Get standard input
    let mut handle = stdin.lock(); // Lock stdin for efficient reading

    while handle.read(&mut buffer).unwrap_or(0) == 1 {
        if buffer[0] as char == 'q' {
            break; // Exit loop when 'q' is pressed
        }
    }
}

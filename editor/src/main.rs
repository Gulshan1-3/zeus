use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use libc::{termios, tcgetattr, tcsetattr, TCSANOW, ECHO, ICANON,ICRNL,IXON,OPOST,ISIG,VMIN,VTIME};


// Global editor configuration struct
struct EditorConfig {
    orig_termios: termios,
}

// Global instance (similar to global 'E' in C)
static mut EDITOR_CONFIG: Option<EditorConfig> = None;


const fn ctrl_key(k: u8) -> u8 {
    k & 0x1f
}

fn editor_draw_rows() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for _ in 0..24 {
        write!(handle, "~\r\n")?;
    }
    
    handle.flush()?;
    Ok(())
}

fn editor_refresh_screen() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    // Clear screen and move cursor to top-left
    write!(handle, "\x1b[2J")?;   // Clear entire screen
    write!(handle, "\x1b[H")?;    // Move cursor to top-left
    
    // Draw rows
    drop(handle);  // Release lock before calling draw_rows
    editor_draw_rows()?;
    
    let mut handle = stdout.lock();
    write!(handle, "\x1b[H")?;    // Move cursor to top-left again
    
    handle.flush()?;
    Ok(())
}

fn enable_raw_mode() -> termios {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = termios { ..unsafe { std::mem::zeroed() } };

    unsafe {
        // Get current terminal settings
        tcgetattr(stdin_fd, &mut termios);
    }

    let original_termios = termios; // Save original settings

    unsafe {
        // Disable ECHO and ICANON (canonical mode)
        termios.c_lflag &= !(ECHO | ICANON);
        termios.c_iflag &= !(ICRNL | IXON);
        termios.c_lflag &= !(ECHO | ICANON | ISIG);
        termios.c_iflag &= !(IXON);
        termios.c_oflag &= !(OPOST);
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 100;
        tcsetattr(stdin_fd, TCSANOW, &termios);
    }

    original_termios
}

fn disable_raw_mode(original: termios) {
    let stdin_fd = io::stdin().as_raw_fd();
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &original);
    }
}

fn main() {
    // Enable raw mode
    let original_termios = enable_raw_mode();

    println!("Raw mode enabled. Press 'ctrl + q' to quit.");

    let mut buffer = [0; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    while handle.read(&mut buffer).unwrap_or(0) == 1 {
        let _ = editor_refresh_screen();
        let b = buffer[0];
        let c = b as char;
        print!("{}", c);
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }
        
        io::stdout().flush().unwrap(); // Ensure immediate output
          
        if buffer[0] == ctrl_key( b'q') {
            break;
        }
    }

    // Restore original mode
    disable_raw_mode(original_termios);

    println!("\nRaw mode disabled.");
}

use std::error::Error;
use std::process::Command;

pub fn read() -> Result<String, String> {
    let output = match Command::new("pbpaste").output() {
        Ok(output) => output,
        Err(e) => return Err(e.description().to_string())
    };

    let s = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return Err("clipboard contains invalid utf8 characters".to_string())
    };

    Ok(s)
}

pub fn write(s: &str) -> Result<(), String> {
    let mut child = match Command::new("pbcopy").spawn() {
        Ok(child) => child,
        Err(e) => return Err(e.description().to_string())
    };

    let stdin = match child.stdin {
        Some(ref mut stdin) =>  stdin,
        None => return Err("Could not write clipboard.".to_string())
    };

    match stdin.write_str(s) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.description().to_string())
    }
}

use std::process::{Stdio, Command};
use std::io::Write;

pub fn read() -> Result<String, String> {
    match Command::new("which").arg("xclip").status() {
        Ok(status) => if !status.success() { return Err("missing xclip program".to_string()) },
        Err(e) => return Err(e.detail().unwrap_or("unknown IO error".to_string()))
    }

    let output = match Command::new("xclip").args(&["-out", "-selection", "clipboard"]).output() {
        Ok(output) => output,
        Err(e) => return Err(e.detail().unwrap_or("unknown IO error".to_string()))
    };

    let s = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return Err("clipboard contains invalid utf8 characters".to_string())
    };

    Ok(s)
}

pub fn write(s: &str) -> Result<(), String> {
    match Command::new("which").arg("xclip").status() {
        Ok(status) => if !status.success() { return Err("missing xclip program".to_string()) },
        Err(e) => return Err(e.detail().unwrap_or("unknown IO error".to_string()))
    }

    let mut child = match Command::new("xclip").args(&["-in", "-selection", "clipboard"]).stdin(Stdio::capture()).spawn() {
        Ok(child) => child,
        Err(e) => return Err(e.detail().unwrap_or("unknown IO error".to_string()))
    };

    let stdin = match child.stdin {
        Some(ref mut stdin) =>  stdin,
        None => return Err("unable to get stdin of xclip".to_string())
    };

    match stdin.write(s.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.detail().unwrap_or("error writing to xclip".to_string()))
    }
}

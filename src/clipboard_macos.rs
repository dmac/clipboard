use std::io::Command;

pub fn read() -> Result<String, String> {
    let output = match Command::new("pbpaste").output() {
        Ok(output) => output,
        Err(e) => return Err(e.detail.unwrap_or("unknown IO error".to_string()))
    };

    let s = match String::from_utf8(output.output) {
        Ok(s) => s,
        Err(_) => return Err("clipboard contains invalid utf8 characters".to_string())
    };

    Ok(s)
}

pub fn write(s: &str) -> Result<(), String> {
    let mut child = match Command::new("pbcopy").spawn() {
        Ok(child) => child,
        Err(e) => return Err(e.detail.unwrap_or("unknown IO error".to_string()))
    };

    let stdin = match child.stdin {
        Some(ref mut stdin) =>  stdin,
        None => return Err("unable to get stdin of pbcopy".to_string())
    };

    match stdin.write_str(s) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.detail.unwrap_or("error writing to pbcopy".to_string()))
    }
}

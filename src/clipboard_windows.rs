use std::io::Command;

pub fn read() -> Result<String, String> {
    unimplemented!()
}

pub fn write(s: &str) -> Result<(), String> {
    let mut child = match Command::new("clip").spawn() {
        Ok(child) => child,
        Err(e) => return Err(e.detail.unwrap_or("unknown IO error".to_string()))
    };

    let stdin = match child.stdin {
        Some(ref mut stdin) =>  stdin,
        None => return Err("unable to get stdin of clip".to_string())
    };

    match stdin.write_str(s) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.detail.unwrap_or("error writing to clip".to_string()))
    }
}


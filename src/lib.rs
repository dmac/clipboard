#![feature(io)]
#![feature(process)]

#[cfg(target_os = "macos")]
#[path = "clipboard_macos.rs"]
mod clipboard;

#[cfg(target_os = "linux")]
#[path = "clipboard_linux.rs"]
mod clipboard;

#[cfg(target_os = "windows")]
#[path = "clipboard_windows.rs"]
mod clipboard;

pub fn read() -> Result<String, String> {
    clipboard::read()
}

pub fn write(s: &str) -> Result<(), String> {
    clipboard::write(s)
}

#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
#[test]
fn test_write_and_read() {
    assert!(write("foo bar").is_ok());
    assert_eq!("foo bar", read().unwrap().as_slice());
}

#[cfg(target_os = "windows")]
#[test]
fn test_write() {
    assert_eq!(Ok(()), write("foo bar"));
}

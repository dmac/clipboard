#[cfg(target_os = "macos")]
#[path = "clipboard_macos.rs"]
mod clipboard;

#[cfg(target_os = "linux")]
#[path = "clipboard_linux.rs"]
mod clipboard;

pub fn read() -> Result<String, String> {
    clipboard::read()
}

pub fn write(s: &str) -> Result<(), String> {
    clipboard::write(s)
}

#[test]
fn test_write_and_read() {
    assert!(write("foo bar").is_ok());
    assert_eq!("foo bar", read().unwrap().as_slice());
}

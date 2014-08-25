A clipboard library for Rust.

## Usage

```rust
extern crate clipboard;

fn main() {
    clipboard::write("foo");
    println!("{}", clipboard::read().unwrap()); // prints "foo"
}
```

## Notes

* Requires xclip to be installed for Linux

## TODO

* Support reading clipboard on Windows

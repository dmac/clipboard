A clipboard library for Rust.

## Usage

    extern crate clipboard;

    fn main() {
        clipboard::write("foo");
        println!("{}", clipboard::read().unwrap()); // prints "foo"
    }

## TODO

* Support Linux
* Support Windows

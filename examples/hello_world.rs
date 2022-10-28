use syslib::*;

pub fn main() {
    write(Fd::stdout, "Hello, World!\n".as_bytes()).unwrap();
}
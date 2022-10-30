use std::os::unix::prelude::OsStrExt;

use syslib::*;

fn main() {
    let mut buffer = [0; 4096];
    write(Fd::stdout, "File: ".as_bytes()).unwrap();
    let path = read(Fd::stdin, &mut buffer).unwrap();
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(path.strip_suffix(b"\n").unwrap()));
    let fd = open(path, open::Flags::CREATE | open::Flags::WRITE_ONLY, mode!(rw_ ___ ___)).unwrap();
    loop {
        let data = read(Fd::stdin, &mut buffer).unwrap();
        if data == b"\x1b\n" { break }
        if write(&fd, data).unwrap() == 0 {
            break
        }
    }
}
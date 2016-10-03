/*
    rust-dos2unix
    Copyright 2016 Sam Saint-Pettersen.

    Released under the MIT license;
    see LICENSE.
*/

use std::fs::File;
use std::io::prelude::*;

pub struct Dos2Unix;

impl Dos2Unix {
    fn is_ascii(contents: String) -> bool {
        let mut ascii = true;
        for c in contents.chars() {
            let code = c as i32;
            if code > 127 {
                ascii = false;
                break;
            }
        }
        ascii
    }

    fn is_dos_eol(contents: String) -> bool {
        let mut dos_eol = false;
        for c in contents.chars() {
            if c == '\r' {
                dos_eol = true;
                break;
            }
        }
        dos_eol
    }

    pub fn convert(filename: &str, feedback: bool, write: bool) -> bool {
        let mut input = File::open(filename).unwrap();
        let mut contents = String::new();
        let _ = input.read_to_string(&mut contents);
        Dos2Unix::is_ascii(contents.clone());
        Dos2Unix::is_dos_eol(contents.clone())
    }
}

#[cfg(test)]
#[test]
fn convert() {
    Dos2Unix::convert("README.md", true, false);
}

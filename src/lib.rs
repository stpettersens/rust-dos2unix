/*
    rust-dos2unix
    Copyright 2016 Sam Saint-Pettersen.

    Released under the MIT license;
    see LICENSE.
*/

//! Rust library to convert DOS to Unix line endings.

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

    fn to_unix_line_endings(contents: String) -> Vec<String> {
        let mut ucontents = Vec::new();
        for c in contents.chars() {
            if c != '\r' {
                ucontents.push(format!("{}", c));
            }
        }
        ucontents
    }

    ///
    /// Convert a file to have Unix line endings.
    ///
    /// * `filename` - Filename to open and change line endings for.
    /// * `feedback` - Display feedback when file already has Unix line endings or is binary.
    pub fn convert(filename: &str, feedback: bool) -> bool {
        let mut input = File::open(filename).unwrap();
        let mut contents = String::new();
        let _ = input.read_to_string(&mut contents);
        let ascii = Dos2Unix::is_ascii(contents.clone());
        let dos_eol = Dos2Unix::is_dos_eol(contents.clone());

        let message = "dos2unix: File already has Unix line endings or is binary.";
        let mut success = false;

        if ascii && dos_eol {
            let converted = Dos2Unix::to_unix_line_endings(contents.clone());
            let mut w = File::create(filename).unwrap();
            let _ = w.write_all(converted.join("").as_bytes());
            success = true;
        } else if feedback {
            println!("{}", message);
        }
        success
    }
}

#[cfg(test)]
#[test]
fn convert() {
    Dos2Unix::convert("README.md", true);
    let mut input = File::open("README.md").unwrap();
    let mut contents = String::new();
    let _ = input.read_to_string(&mut contents);
    let mut has_dos_eol = false;
    for c in contents.chars() {
        if c == '\r' {
            has_dos_eol = true;
            break;
        }
    }
    assert_eq!(has_dos_eol, false);
}

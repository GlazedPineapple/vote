use std::{env, fs};

use regex::Regex;

fn main() {
    if fs::metadata(".env").is_ok() && env::var("CI").is_err() {
        println!("cargo:rerun-if-changed=.env");

        let contents = fs::read_to_string(".env").expect("Failed to read .env");

        fs::write(
            "example.env",
            Regex::new(r#"".*" # Private"#)
                .unwrap()
                .replace_all(&contents, "")
                .as_bytes(),
        )
        .expect("Failed to write example.env");
    }
}

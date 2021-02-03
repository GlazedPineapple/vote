use std::{env, fs, io, path::Path};

use regex::Regex;

fn main() -> io::Result<()> {
    if env::var_os("CI").is_none() {
        let dotenv = Path::new(".env");
        let example_dotenv = Path::new("example.env");

        if dotenv.exists() {
            println!("cargo:rerun-if-changed=.env");

            if example_dotenv.exists() {
                let dotenv_meta = fs::metadata(dotenv)?;
                let example_dotenv_meta = fs::metadata(example_dotenv)?;

                if dotenv_meta.modified()? < example_dotenv_meta.modified()? {
                    eprintln!(".env file is out of date, not updating example.env");

                    return Ok(());
                }
            }

            // Create example dotenv

            let contents = fs::read_to_string(dotenv)?;

            fs::write(
                example_dotenv,
                Regex::new("[\"'].*[\"']")
                    .unwrap()
                    .replace_all(&contents, "")
                    .as_bytes(),
            )
            .expect("Failed to write example.env");
        } else {
            eprintln!(".env file missing! It is recomended to create one based on the example.env")
        }
    }

    Ok(())
}

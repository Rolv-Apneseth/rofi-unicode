//! Fetches and converts the `UnicodeData.txt` file from the below URL to a more useable format.

use std::{
    error::Error,
    fs::{File, remove_file},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, serde::Deserialize)]
struct Record {
    code: String,
    name: String,
    _a: Option<String>,
    _b: Option<String>,
    _c: Option<String>,
    _d: Option<String>,
    _e: Option<String>,
    _f: Option<String>,
    _g: Option<String>,
    _h: Option<String>,
    alt_name: String,
    _i: Option<String>,
    _j: Option<String>,
    _k: Option<String>,
}

fn make_ascii_titlecase(s: &mut str) {
    s.make_ascii_lowercase();
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let resp =
        reqwest::blocking::get("https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt")?
            .error_for_status()?;

    let text = resp.text()?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(text.as_bytes());

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("src")
        .join("unicode.rs");

    if path.exists() {
        remove_file(&path)?;
    }

    let mut file = File::create_new(path)?;
    file.write_all(b"// Automatically generated - don't edit this file directly\n")?;
    file.write_all(indoc::indoc! {
    b"
    pub struct Unicode {
        pub char: &'static str,
        pub name: &'static str,
        pub alt_name: &'static str,
    }

    impl Unicode {
        const fn new(char: &'static str, name: &'static str, alt_name: &'static str) -> Self {
            Self { char, name, alt_name }
        }
    }

    "
    })?;
    file.write_all(b"pub const UNICODE: &[Unicode] = &[\n")?;

    for res in rdr.deserialize() {
        let mut record: Record = res?;

        // Ignore certain groups
        if record.name.contains("<control>")
            || record.name.contains("Surrogate")
            || record.name.contains("VARIATION SELECTOR")
            || record.name.contains("PRIVATE USE")
            || record.name.contains("Private Use")
            || record.name.contains("COMBINING")
        {
            continue;
        }

        make_ascii_titlecase(&mut record.name);
        record.alt_name.make_ascii_lowercase();

        file.write_all(
            format!(
                "    Unicode::new(\"\\u{{{}}}\", \"{}\", \"{}\"),\n",
                record.code, record.name, record.alt_name
            )
            .as_bytes(),
        )?;
    }

    file.write_all(b"];\n")?;

    Ok(())
}

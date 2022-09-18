#![allow(clippy::needless_doctest_main)]
//! Reads key value pairs from an .env or any other file and stores them
//! as easily available environment variables.
//! Since dotenv is no longer maintained, this is an simpler smaller alternative.
//! # Example
//! ```rust
//! use stupid_simple_dotenv::to_env;
//!
//! fn main() {
//!    to_env().ok();
//!    println!("Hello, {}!", std::env::var("myuser").unwrap());// in .env file: myuser=world
//! }
//!

use std::{fs::File, io::BufRead, path::Path};
/// Reads .env file stores the key value pairs as environment variables.
/// ```rust
/// fn main() {
///    stupid_simple_dotenv::to_env(); // reads .env file and stores the key value pairs as environment variables
///    let value = std::env::var("myuser").unwrap(); //Works if key value pair is present in .env file
/// }
///
/// ```
pub fn to_env() -> Result<(), Box<dyn std::error::Error>> {
    let list = read(".env")?;
    for line in list {
        let (key, value) = (line.0, line.1);
        std::env::set_var(key, value);
    }
    Ok(())
}

pub fn to_vec() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let list = read(".env")?;
    Ok(list)
}

/// Reads named file stores the key value pairs as environment variables.
/// ```rust
/// fn main() {
///     stupid_simple_dotenv::file_to_env("other.env");
///     let value = std::env::var("other_user").unwrap();
///     assert_eq!(value, "other user name");
/// }
pub fn file_to_env<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let list = read(path)?;
    for line in list {
        let (key, value) = (line.0, line.1);
        std::env::set_var(key, value);
    }
    Ok(())
}

/// Reads key value pairs from a file and returns a vector of tuples.
/// ```rust
/// fn main() {
///     let list = stupid_simple_dotenv::file_to_vec("other.env").unwrap(); // reads other.env file and stores the key value pairs as environment variables
///     for item in list{
///         println!("Key:{}, Value:{}", item.0, item.1);
///     }
/// }
pub fn file_to_vec<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let list = read(path)?;
    Ok(list)
}

/// Try to get the value of an environment variable.
/// If the variable is not present in the environment, `default` is returned.
/// ```rust
/// fn main() {
///     let value = stupid_simple_dotenv::get_or("key_not_here", "default_key");
///     println!("{}", &value);
///     assert_eq!("default_key", &value);
/// }
pub fn get_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}
fn read<P: AsRef<Path>>(path: P) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let lines = std::io::BufReader::new(f).lines();
    parse(lines)
}

fn parse(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    let lines = lines;
    for (col, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let parsed = match parse_line(line) {
            Ok(parsed) => parsed,
            Err(e) => {
                return Err(format!("Error parsing line {}. {}", col + 1, e).into());
            }
        };
        list.push((parsed.0.to_owned(), parsed.1.to_owned()));
    }
    Ok(list)
}
fn parse_line(s: &str) -> Result<(&str, &str), Box<dyn std::error::Error>> {
    let mut parts = s.splitn(2, '=');
    let key = match parts.next() {
        Some(key) => key.trim_end(),
        None => return Err(format!("No key found in line '{}'", s).into()),
    };

    let value = match parts.next() {
        Some(value) => value.trim_start(),
        None => return Err(format!("No value found in line '{}'", s).into()),
    };
    let value = remove_quotes(value);
    Ok((key, value))
}

fn remove_quotes(s: &str) -> &str {
    if s.starts_with('"') && s.ends_with('"')
        || s.starts_with('\'') && s.ends_with('\'')
        || s.starts_with('`') && s.ends_with('`')
    {
        return &s[1..s.len() - 1];
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        to_env().unwrap();
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("FOO=BAR").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO = BAR").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=\"BAR\"").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO='BAR'").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=`BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=\t `BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO\t=\t `BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO\t=\t ` BAR`").unwrap(), ("FOO", " BAR"));
        assert_eq!(parse_line("FOO\t=\t ` BAR `").unwrap(), ("FOO", " BAR "));
        assert_eq!(
            parse_line("FOO\t   =   \t ` BAR `").unwrap(),
            ("FOO", " BAR ")
        );
        assert_eq!(
            parse_line(" FOO\t   =   \t ` BAR `").unwrap(),
            (" FOO", " BAR ")
        );
    }

    #[test]
    fn test_remove_quotes() {
        assert_eq!(remove_quotes("BAR"), "BAR");
        assert_eq!(remove_quotes("\"BAR\""), "BAR");
        assert_eq!(remove_quotes("'BAR'"), "BAR");
        assert_eq!(remove_quotes("`BAR`"), "BAR");
        assert_eq!(remove_quotes(" `BAR`"), " `BAR`");
        assert_eq!(remove_quotes(" ` BAR`"), " ` BAR`");
        assert_eq!(remove_quotes(" ` BAR `"), " ` BAR `");
    }

    #[test]
    fn test_parse() {
        let env_sim = r#"
FOO=BAR
# comment
FOO2= BAR2

FOO3="BAR3"
FOO4='BAR4'
FOO5=`BAR5`
"#;
        let lines = env_sim.lines().map(|s| Ok(s.to_owned()));
        let list = parse(lines).unwrap();
        assert_eq!(
            list,
            vec![
                ("FOO".to_owned(), "BAR".to_owned()),
                ("FOO2".to_owned(), "BAR2".to_owned()),
                ("FOO3".to_owned(), "BAR3".to_owned()),
                ("FOO4".to_owned(), "BAR4".to_owned()),
                ("FOO5".to_owned(), "BAR5".to_owned()),
            ]
        );
    }
}

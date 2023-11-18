#![allow(clippy::needless_doctest_main)]
//! Reads key-value pairs from a file, such as an .env file, and makes them
//! easily accessible as environment variables. This crate provides a simpler and smaller
//! alternative to dotenv, which is no longer maintained.
//! # Example
//! ```rust
//! use stupid_simple_dotenv::to_env;
//!
//! fn main() ->Result<(), Box<dyn std::error::Error>> {
//!    match to_env(){
//!       Ok(_) => println!("Success reading .env"),
//!       Err(e) if e.kind == "io" =>{
//!         println!("IO-Error better not ignore! {}", e);
//!         // you can return the Error: return Err(e.into());
//!       },
//!       Err(e) if e.kind == "LinesError" => {
//!         println!("Errors in some lines of .env: {}", e);
//!       },
//!       Err(e) => {
//!         println!("Error {}", e);
//!         // You can return the Error return Err(e.into());
//!       },
//!    };
//!    let user = std::env::var("USER")?; // if USER is not set, this will return an error
//!    println!("USER: {}", user);
//!    Ok(())
//! }
//!

use std::error::Error as StdError;
use std::{error::Error, fmt::Display, fs::File, io::BufRead, path::Path};
#[derive(Debug)]
pub struct SimpleEnvError {
    pub kind: String,
    message: String,
    pub list: Option<Vec<(String, String)>>,
}

impl Display for SimpleEnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}
impl Error for SimpleEnvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl From<std::io::Error> for SimpleEnvError {
    fn from(error: std::io::Error) -> Self {
        SimpleEnvError {
            kind: String::from("io"),
            message: error.to_string(),
            list: None,
        }
    }
}

pub struct SimpleEnvErrorWrapper(SimpleEnvError);

impl From<SimpleEnvError> for SimpleEnvErrorWrapper {
    fn from(error: SimpleEnvError) -> Self {
        SimpleEnvErrorWrapper(error)
    }
}

impl Into<Box<dyn StdError>> for SimpleEnvErrorWrapper {
    fn into(self) -> Box<dyn StdError> {
        Box::new(self.0)
    }
}
impl From<SimpleEnvError> for std::io::Error {
    fn from(err: SimpleEnvError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

/// Reads .env file stores the key value pairs as environment variables.
/// ```rust
/// fn main() {
///    let _ = stupid_simple_dotenv::to_env(); // reads .env file and stores the key value pairs as environment variables
///    let value = std::env::var("key").expect("Key 'key' not found."); //Works if key value pair is present in .env file
/// }
///
/// ```
pub fn to_env() -> Result<(), SimpleEnvError> {
    match read(".env") {
        Ok(list) => {
            iter_to_env(&list);
            Ok(())
        }
        Err(e) => {
            e.list.as_ref().map(iter_to_env);
            Err(e)
        }
    }
}

fn iter_to_env(list: &Vec<(String, String)>) {
    for line in list {
        let (key, value) = (&line.0, &line.1);
        std::env::set_var(key, value);
    }
}
/// Reads .env file to a vector of key value pairs tuples.
/// ```rust
/// fn main() {
///     let list = match stupid_simple_dotenv::to_vec(){
///         Ok(list) => list,
///         Err(e) => {
///             println!("Error {}", e);
///             if let Some(list) = e.list{
///                 list
///             }else{
///                 panic!("{}",e.to_string());
///             }
///         },
///     }; // reads .env file to a vector of key value pairs tuples
///     for line in list {
///         println! ("Key:{}, Value:{}",line.0, line.1);
///     }
/// }
/// ```
pub fn to_vec() -> Result<Vec<(String, String)>, SimpleEnvError> {
    let list = read(".env")?;
    Ok(list)
}

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
///     assert_eq!("default_key", &value);
/// }
pub fn get_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn read<P: AsRef<Path>>(path: P) -> Result<Vec<(String, String)>, SimpleEnvError> {
    let f = File::open(path)?;
    let lines = std::io::BufReader::new(f).lines();
    parse(lines)
}

fn parse(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> Result<Vec<(String, String)>, SimpleEnvError> {
    let mut error_lines = Vec::new();
    let mut num_error_lines = 0;
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
                num_error_lines += 1;
                if error_lines.len() < 10 {
                    error_lines.push(format!("Error in Line {col}: {e}"));
                }
                error_lines.push(format!("Error in Line {col}: {e}"));
                continue;
            }
        };
        list.push((parsed.0.to_owned(), parsed.1.to_owned()));
    }
    if error_lines.is_empty() {
        Ok(list)
    } else {
        if num_error_lines > error_lines.len() {
            error_lines.push(format!(
                "And {} more errors in .env file",
                num_error_lines - error_lines.len()
            ));
        }
        Err(SimpleEnvError {
            kind: "LinesError".to_string(),
            message: error_lines.join("\n"),
            list: Some(list),
        })
    }
}

fn parse_line(s: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let mut name_begin: usize = 0;
    let mut name_end: usize = 0;
    let mut value_begin: usize = 0;
    let mut value_end: usize = 0;
    let mut in_name = true;
    let mut in_value = false;
    let mut quotes = 'f';
    let mut must_trim = false;
    for (pos, c) in s.char_indices() {
        match c {
            '"' | '\'' | '`' => {
                if quotes != 'f' {
                    //We are in Quotes
                    if quotes == c {
                        //End of Quotes
                        quotes = 'f';
                        if in_name {
                            name_end = pos - 1;
                        }
                        if in_value {
                            value_end = pos - 1;
                            break; //Done
                        }
                    }
                } else {
                    //We are not in Quotes, let it begin
                    quotes = c;
                    if in_name {
                        name_begin = pos + 1;
                    }
                    if in_value {
                        value_begin = pos + 1;
                    }
                }
            }
            '=' => {
                if quotes != 'f' {
                    continue;
                }
                if in_name {
                    in_name = false;
                    in_value = true;
                    if name_end == 0 && pos > 0 {
                        name_end = pos - 1;
                    }
                }
            }
            '#' => {
                if quotes != 'f' {
                    continue;
                }
                if in_value {
                    value_end = pos - 1;
                    must_trim = true;
                    break;
                }
            }
            _ => {
                if in_name {
                    if c.is_whitespace() && quotes == 'f' {
                        continue;
                    }
                    name_end = pos;
                } else if c.is_whitespace() && quotes == 'f' {
                    continue;
                }
                if in_value {
                    value_end = pos;
                    if value_begin == 0 {
                        value_begin = pos;
                    }
                }
            }
        }
    }
    if value_begin == 0 || name_end == 0 {
        Err(format!("No name or value in '{s}'").into())
    } else if value_begin == 0 {
        Err("No value".into())
    } else {
        if must_trim {
            {
                let s = &s[value_begin..=value_end];
                value_end = value_begin + s.trim_end().len() - 1;
            }
        }
        Ok((&s[name_begin..=name_end], &s[value_begin..=value_end]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        match to_env() {
            Ok(_) => {
                //Error expected. Sample .env file has error
                assert!(false)
            }
            Err(e) => {
                //OK
                assert_eq!(e.kind, "LinesError");
                assert_eq!(
                    e.message
                        .starts_with("Error in Line 14: No name or value in 'error='"),
                    true
                );
            }
        }
    }

    #[test]
    fn test_parse_line_new() {
        assert_eq!(parse_line("FOO=BAR").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("\"FOO\"=\"BAR\"").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO = BAR").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=\"BAR\"").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO='BAR'").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=`BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO=\t `BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO\t=\t `BAR`").unwrap(), ("FOO", "BAR"));
        assert_eq!(parse_line("FOO\t=\t ` BAR`").unwrap(), ("FOO", " BAR"));
        assert_eq!(
            parse_line("FOO\t=\t ` BAR`#comment").unwrap(),
            ("FOO", " BAR")
        );
        assert_eq!(parse_line("FOO\t=\t ` BAR `").unwrap(), ("FOO", " BAR "));
        assert_eq!(
            parse_line("FOO\t   =   \t ` BAR `").unwrap(),
            ("FOO", " BAR ")
        );
        assert_eq!(
            parse_line(" FOO\t   =   \t ` BAR `").unwrap(),
            (" FOO", " BAR ")
        );

        assert_eq!(true, matches!(parse_line(" FOO\t   = "), Result::Err(_)));
        assert_eq!(true, matches!(parse_line(" FOO\t   ="), Result::Err(_)));
        assert_eq!(true, matches!(parse_line("="), Result::Err(_)));
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
FOO6=BAR6 #comment
Foo7=BA😀R7 #comment
"#;
        let lines = env_sim.lines().map(|s| Ok(s.to_owned()));
        let lines_clone = lines.clone();
        let start = std::time::Instant::now();
        let list = parse(lines).unwrap();
        let time_old = start.elapsed();
        let start = std::time::Instant::now();
        let list2 = parse(lines_clone).unwrap();
        println!("Old: {:?} New {:?}", time_old, start.elapsed());
        assert_eq!(
            list,
            vec![
                ("FOO".to_owned(), "BAR".to_owned()),
                ("FOO2".to_owned(), "BAR2".to_owned()),
                ("FOO3".to_owned(), "BAR3".to_owned()),
                ("FOO4".to_owned(), "BAR4".to_owned()),
                ("FOO5".to_owned(), "BAR5".to_owned()),
                ("FOO6".to_owned(), "BAR6".to_owned()),
                ("Foo7".to_owned(), "BA😀R7".to_owned()),
            ]
        );
        assert_eq!(list, list2);
    }
}

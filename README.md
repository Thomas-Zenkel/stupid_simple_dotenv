# Stupid simple dotenv

A simple dotenv parser for Rust with no dependencies.

Reads key value pairs from an .env or any other file and stores them
as easily available environment variables.
Since dotenv is no longer maintained, this is an simpler smaller alternative.

## Breaking Changes in 0.3.0

**`to_env()` and `file_to_env()` no longer override existing environment variables.**

This aligns with the standard dotenv behavior where shell exports take precedence over `.env` file values.
If you need the old behavior of overriding existing variables, use the new `to_env_override()` or `file_to_env_override()` functions.

```rust
// Standard behavior: does NOT override existing env vars
stupid_simple_dotenv::to_env().ok();

// Override behavior: DOES override existing env vars (old behavior)
stupid_simple_dotenv::to_env_override().ok();
```

## Usage

```rust
use stupid_simple_dotenv;
fn main() {
    // Reads .env file - does NOT override existing environment variables
    stupid_simple_dotenv::to_env().ok();
    println!("Hello, {}!", std::env::var("myuser").unwrap()); // Hello, world!

    // Use to_env_override() if you want to override existing variables
    stupid_simple_dotenv::to_env_override().ok();

    // Read from a custom file - does NOT override existing variables
    stupid_simple_dotenv::file_to_env("other.env").ok();

    // Read from a custom file - DOES override existing variables
    stupid_simple_dotenv::file_to_env_override("other.env").ok();

    println!(
        "Hello, {}!",
        stupid_simple_dotenv::get_or("other_user", "Not set")
    ); // Hello, other user name!

    println!(
        "Hello, {}!",
        stupid_simple_dotenv::get_or("other_user_not_set", "not set")
    ); // Hello, not set!

    let list = stupid_simple_dotenv::file_to_vec("other.env").unwrap();
    let other_user_name = list.iter().find(|(key, _value)| key == "other_user");
    if let Some((_, value)) = other_user_name {
        println!("Hello, {}!", value); // Hello, other user name!
    }
}
```

## API

| Function | Description |
|----------|-------------|
| `to_env()` | Reads `.env` file, sets variables that don't exist yet |
| `to_env_override()` | Reads `.env` file, overrides all variables |
| `file_to_env(path)` | Reads custom file, sets variables that don't exist yet |
| `file_to_env_override(path)` | Reads custom file, overrides all variables |
| `to_vec()` | Reads `.env` file to a vector of key-value tuples |
| `file_to_vec(path)` | Reads custom file to a vector of key-value tuples |
| `get_or(key, default)` | Gets env var or returns default value |

## Valid .env file

.env files are simply configuration files in which a key-value pair separated by a = character is specified per line.
The keys are to be specified without quotes.
The values can be specified also without, however, are permissible also: ", ' and ` as quotes.
Comments are to be introduced with # and must stand in a new line. Comments after values are not recognized and interpreted as values. Spaces before or after keys and values are ignored. If they are intended, quotes must be used.
Valid lines of an .env file for this parser are:

```env
myuser = world
other_user = other user name
key0=value
 key1= value
key2=value
key3='value'
key4=`value`
key5=` value with spaces `
 #comment
key6 = value with spaces inside
key 7= value:with#special&UTF-8 Chars
key-8="value:with#special&UTF-8 Chars_and_quote"
key9="comments now enabled in the same Line" #this is a comment and will be ignored
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

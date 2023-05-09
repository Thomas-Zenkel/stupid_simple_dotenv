# Stupid simple dotenv

A simple dotenv parser for Rust with no dependencies.

Reads key value pairs from an .env or any other file and stores them
as easily available environment variables.
Since dotenv is no longer maintained, this is an simpler smaller alternative.

## Usage
```rust
use stupid_simple_dotenv;
fn main() {
    stupid_simple_dotenv::to_env().ok();
    println!("Hello, {}!", std::env::var("myuser").unwrap()); // Hello, world!

    stupid_simple_dotenv::file_to_env("other.env").ok();
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
key 7= value:with#special&😀UTF-8 Chars🦀
key-8="value:with#special&😀UTF-8 Chars🦀and_quote"
key9="comments now enabled in the same Line" #this is a comment and will be ignored
```
## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

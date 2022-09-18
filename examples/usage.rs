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

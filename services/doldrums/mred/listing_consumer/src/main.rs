use substring::{find_substring, replace_substring};

fn main() {
    let s = "Hello, World!";

    test_it().unwrap();

    let substring = find_substring(s, "World");
    println!("Found substring: {:?}", substring);

    let new_string = replace_substring(s, "World", "Rust");
    println!("New string: {}", new_string);
}

fn test_it() -> anyhow::Result<()> {
    Ok(())
}

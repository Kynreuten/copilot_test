use std::collections::HashMap;


// write a main function in rust that takes in key/value pairs of strings as arguments from the command line and puts them into a Vector. The arguments will each be in the format "key:value". Then prints out each pair on a separate line.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut map = HashMap::new();

    for arg in &args[1..] {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        } else {
            println!("Invalid argument: {}. Expected format of K/Vs is \"key:value\"", arg);
            return;
        }
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

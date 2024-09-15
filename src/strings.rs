use std::io::{self, Write}; /// In order to write

/// Main function
/// Wherever this is, it is documentation text
/// I'm still learning how to use this
/// Scaaaarrredddd :fearful:

pub fn main(){
    let mut input = String::new(); // New input mutable string initialized to empty
    print!("Enter your first name: ");
    io::stdout().flush().unwrap(); // Flush the output buffer
    io::stdin().read_line(&mut input).unwrap(); // Read the line from stdin and unwrap the result as string, read_line returns a Result, and needs to table &mut to update input var with user input
    let first_name = input.trim().to_string(); // Trim the input and convert to string

    input.clear(); // Clear the input buffer

    print!("Enter your last name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let last_name = input.trim().to_string();

    input.clear();

    print!("Enter your age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let age: &str = input.trim();

    let fullname = format!("{} {}", first_name, last_name); 
    let message = format!("Hello, {}! You are {} years old.", fullname, age);

    println!("{}", fullname);
    println!("{}", message);
}


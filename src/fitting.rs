use std::io;

pub fn calculate_fit(){
    println!("Something is getting rusty here 🚀");
    let mut input = String::new();
    println!("Enter your chest measure in cm: ");
    let some_n: usize= io::stdin().read_line(&mut input).unwrap();
    println!("This is extra hahahah");
    println!("There are {} bytes in this input. Very useful ..", some_n);
    println!("Is this input? {}", input);
}
fn main() {
    // Example 1: Index out of bounds
    let v = vec![1, 2, 3];
    println!("The fourth element is: {}", v[3]); // This will panic

    // Example 2: Unwrapping a None value
    let option: Option<i32> = None;
    println!("The value is: {}", option.unwrap()); // This will panic

    // Example 3: Division by zero
    // let x = 10;
    // let y = 0;
    // println!("The result is: {}", x / y); // This will panic
}
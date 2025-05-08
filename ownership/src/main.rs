// Chapter 4: Ownership

// Ownership introduced by rust to solve memory safety issues 
// and high performance at the same time

// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner]


// Ownership Rules
// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


// Example of Rule 1: Each value in Rust has an owner
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Example of Rule 2: There can only be one owner at a time
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     println!("S2 value is {}.", s2);
// }

// Example of Rule 3: When the owner goes out of scope, the value will be dropped
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("The length of {} is {}.", s1, len);
// }

// fn printLost(s: &String) {
//     println!("{}", &s1);
// }
// you will get error[E0425]: cannot find value `s1` in this scope

// s1 goes out of the scope and its value will be dropped

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// Chapter 7: Constants

// Rules of constants

// 1. Cannot use mut keyword. Constant cannot be mutated
// 2. Declaring constants will always be in capital letters

// You can declare constant out of scope can be always accessed.

fn main() {
    println!("Chapter 7: Constants");
    // let mut x = 5;
    // const mut Y = 10; // Error constant cannot mutable

    // println!("X is {}", x);
    // println!("Y is {}", Y)

    const PI: f32 = 3.14;
    println!("Pi value is {}.", PI);

    println!("Printing constant declared in out of scope, Rage is {}", RAGE);

    println!("3 Hours in seconds: {} seconds", THREE_HOURS_IN_SECONDS)
}

const RAGE: i32 = 10;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
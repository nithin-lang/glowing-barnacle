// Chapter 6: Variables and Mutability

fn main() {
    // let a: i32 = 8;
    // println!("A value is {}", a);

    // a = 10;
    // error[E0384]: cannot assign twice to immutable variable `a`

// add mut keyword before variable allows mutability

    let mut a: i32 = 8;
    println!("A value is {}", a);

    a = 10;
    println!("A value now is {}", a);
}

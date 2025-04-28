// Chapter 2: Compound Data Types

// Compound Data Types
// arrays, tuples, slices and strings (slice string)


fn main() {


    //Array
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Array numbers: {:?}", numbers);

    // Error
    // let mix: [i32; 5] = [1,2,"Apple",false,5];
    // println!("Mix: {:?}", mix);
}

// Chapter 2: Compound Data Types

// Compound Data Types
// arrays, tuples, slices and strings (slice string)


fn main() {


    //Array
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Array numbers: {:?}", numbers);

    // Array Error
    // let mix: [i32; 5] = [1,2,"Apple",false,5];
    // println!("Mix: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits in the basket contain: {:?}", fruits);

    // Get first element in the array
    println!("First fruit in the array: {}", fruits[0]);

    // Tuples

    let human: (String, i32, bool) = ("Punisher".to_string(), 40, true);
    println!("Human tuple {:?}", human);

    // Mix Tuple

    // without type explicitly declared
    let mix_tuple = ("Punisher".to_string(), 40, true, [1,2,3,4,5]);

    // with type explicitly declared
    // let mix_tuple: (String, i32, bool, [i32; 5]) = ("Punisher".to_string(), 40, true, [1,2,3,4,5]);
    println!("Mix tuple {:?}", mix_tuple);

    // Slices
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Panther", "Jaguar"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Game of Thrones".to_string(), &"Harry Potter".to_string(), &"Rich Dad Poor Dad".to_string()];
    println!("Book slices: {:?}", book_slices);

    // Strings vs String slices (&str)
    // Strings [ growable, mutable, owned string type]
    let mut stone_cold:String = String::from("Hell, ");
    println!("String: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("String: {}", stone_cold);

    // &str (String Slice)
    let numerics: String = String::from("Hello World!");
    let slice: &str = &numerics[0..5];
    println!("Slice value: {}", slice);
}

fn main() {
    my_name();
    my_weight(45);
    human_id("Barnacle", 45, 45.8);

    // Expressions
    let _x: i32 = {
        let price: i32 = 5;
        let quantity: i32 = 30;
        price * quantity
    };
    println!("_x value is: {}", _x);
    let y: i32 = add(1990, 2989);
    println!("y value is: {}", y);

    // Passing the expression as a value
    println!("Add value is: {}", add(20, 70));

    // BMI calculation
    let weight = 70.0;
    let height = 1.75;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi); 

}

fn my_name() {
    let name: String = String::from("Barnacle");
    println!("My Name is {}", name);
}

fn my_weight(weight: i32) {
    println!("My Weight is {} kg(s)", weight);
}

// Multiple value
fn human_id(name: &str, age: i32, weight: f32) {
    println!("My name is {}, I am {} years old and my weight is {} kg(s)", name, age, weight);
}

// functions returing values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// BMI function
fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg/(height_m * height_m)
}
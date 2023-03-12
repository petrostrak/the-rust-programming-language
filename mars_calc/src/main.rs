use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    let weight: f32 = user_input.trim().parse::<f32>().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {:.2}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}

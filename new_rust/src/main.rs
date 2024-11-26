use std::io;
// use rand::Rng;
// use std::cmp::*;
fn main() {
    println!("Enter a temperature to convert the temperature");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("enter a number");

    let number: f32 = temp.trim().parse().expect("enter a valid number");
    
    let temp_in_far = celcius_to_far(number);

    let temp_in_celcius = far_to_celcius(number);

    println!("{}° in farenheit is {}° in celcius",number,temp_in_celcius);
    println!("{}° in celcius is {}° in farenheit",number,temp_in_far);
}

fn celcius_to_far(celcius: f32) -> f32 {
    32 as f32 + (celcius * 9 as f32 / 5 as f32) 
}
fn far_to_celcius(far: f32) -> f32 {
    (far - 32 as f32)  * 5 as f32 / 9 as f32
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // This is an array of struct type User which carries the user name and user id
    let user_details: [User; 3] = [
        User {
            name: "Timileyin".to_string(),
            id: 1,
        },
        User {
            name: "Joseph".to_string(),
            id: 2,
        },
        User {
            name: "STalin".to_string(),
            id: 3,
        },
    ];
    
    //This creates an empty mutable string instance to store our inputted guessed number
    let mut value = String::new();

    let winner_id = rand::thread_rng().gen_range(0..=2); //Randomly generated number
    println!("the winning id is {}", winner_id);
    println!("Enter to check if you are the winner");

    io::stdin().read_line(&mut value).expect("Enter a number"); // input for the cli

    let value_num: usize = value.trim().parse().expect("Pls input a valid number"); //This transforms value from a string to a u32

    // to save the value of the user_details to get direct access to user details struct
    let position = &user_details[value_num];

    match value_num.cmp(&winner_id) {
        //Comparing the inputted number with the random generated number
        // When it is equal, it outputs a result that shows the winner name and id
        Ordering::Less => println!("It is less"),
        Ordering::Equal => println!("Congrats you are the winner, your name is {:#?}, with Id number {:#?}", position.name,position.id),
        Ordering::Greater => println!("It is more"),
    }
}

#[derive(Debug)]
pub struct User {
    name: String,
    id: u32,
}

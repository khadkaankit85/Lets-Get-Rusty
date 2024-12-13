use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[allow(dead_code)]
pub fn guess_game() {
    println!("Guess a Number");
    let mut guess = String::new();

    let my_random_number: i32 = rand::thread_rng().gen_range(1..3);

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the line");

    match guess.cmp(&my_random_number.to_string()) {
        Ordering::Less => {
            println!(
                "your guess is {} the actual number is {}",
                guess, my_random_number
            )
        }
        Ordering::Equal => {
            println!(
                "your guess is {} the actual number is {}",
                guess, my_random_number
            );
        }
        Ordering::Greater => {
            println!(
                "your guess is {} the actual number is {}",
                guess, my_random_number
            )
        }
    }
}

use std::io;
#[allow(dead_code)]

pub fn functions() {
    //let myname=(let name="angkit Khadka");
    //println!("{}", myname);
    fn five() -> i8 {
        5
    }
    //    println!("and the number is {}", five());

    println!("Enter a number to get it back:");
    let mut user_input_number = String::new();
    io::stdin()
        .read_line(&mut user_input_number)
        .expect("Error reading the user input value");

    let user_input_number: i32 = user_input_number
        .trim()
        .parse()
        .expect("Are you sure that is a number?");

    let turnery_practice = if user_input_number == 0 {
        "you are good"
    } else {
        "not good"
    };
    println!("{}", turnery_practice);

    if user_input_number < 5 {
        println!(
            "The number is less than 5 and the number is {}",
            &user_input_number
        );
    } else if user_input_number > 5 && user_input_number < 10 {
        println!(
            "Hi The number is greater than 5 and the number is {}",
            user_input_number
        )
    } else {
        println!("The number is 5 and the number is {}", user_input_number);
    }
}

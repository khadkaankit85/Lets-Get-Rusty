use std::{io, usize};
#[allow(unused_variables)]
#[allow(dead_code)]

pub fn variables() {
    let name: &str = "Angkit";
    //println!("my name is {}", name);
    let name: &str = "Angkit Khdka";
    //println!("my name is {}", name);

    let pi: f32 = 3.14;
    let pi1 = 3.14 as f32;
    let pi2 = 3.14_f32;

    let boolean: bool = false;
    //println!("boolean is {}", boolean);

    let mychar = 'a';

    let mut my_tuple = ("a", "ankit", 143, "aaru");
    my_tuple.0 = "Hello";
    //println!("{}", my_tuple.0);

    let my_array = [1, 2, 3, 4, 5];
    //println!("priting from array {}", my_array[0]);

    let my_arr = [0; 6];
    println!("array is {}", stringify!(my_array));

    let weekdays = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    println!("Enter the day you would wanna know:");

    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("unable to read the index");

    let index_from_user: usize = day.trim().parse().expect("error getttin the index");

    println!(
        "the day you are looking for is {}",
        weekdays[index_from_user]
    );
}

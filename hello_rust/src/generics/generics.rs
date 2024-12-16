/* without generic */
/*
fn larget_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for chars in list {
        if chars > largest {
            largest = chars;
        }
    }
    &largest
}
pub fn generics() {
    let mylist = vec![1, 3, 4, 5];
    let largest = larget_i32(&mylist);
    println!("largest {}", largest);
    let characters = vec!['a', 'b', 'c', 'd'];
    let largest_char = largest_char(&characters);
    println!("largest char {}", largest_char);
}
*/
/*with generics*/

/*
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

pub fn generics() {
    let mylist = vec![1, 3, 4, 5];
    let characters = vec!['a', 'b', 'c', 'd'];
    let largest_char = largest(&characters);
    let largest_i32 = largest(&mylist);
    println!("{largest_char} is the largest char and {largest_i32} is the largest integer");
}
*/

struct Point<T> {
    x: T,
    y: T,
}

pub fn generics() {
    let integer = Point { x: 5, y: 2 };
    let floats = Point { x: 1.2, y: 3.4 };
    println!("integer: x is {} and y is {}", floats.x, floats.y);
    println!("integer: x is {} and y is {}", integer.x, integer.y);
}

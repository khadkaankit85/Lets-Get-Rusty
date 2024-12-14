#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        _ => 0,
    }
}
#[allow(dead_code)]
pub fn matching() {
    let one_penny = Coin::Penny;
    println!("Value of penny is {}", value_in_cents(one_penny));
}

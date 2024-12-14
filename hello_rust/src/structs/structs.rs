#[derive(Debug)]
#[allow(dead_code)]
struct User {
    first_name: String,
    email: String,
    sign_in_count: u64,
    girlfriend_count: i32,
}
#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
pub fn structs() {
    let user1 = User {
        first_name: "Angkit Khadka".to_string(),
        email: "khadkaangkit".to_string(),
        sign_in_count: 69,
        girlfriend_count: 1,
    };
    println!(
        " {} {} {} {}",
        user1.first_name, user1.email, user1.sign_in_count, user1.girlfriend_count
    );
    let red = Color(255, 255, 255);
    //println!("{:#?}", red);
    println!("Red: {} {} {}", red.0, red.1, red.2);
}

#[allow(dead_code)]
/*
enum IpAddress {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddress,
    value: String,
}
fn route(ip_kind: IpAddress) {}

pub fn enums() {
    let home = IpAddr {
        kind: IpAddress::V4,
        value: "127.0.0.1".to_string(),
    };
    let not_home = IpAddr {
        kind: IpAddress::V6,
        value: "::1".to_string(),
    };
}
*/
#[derive(Debug)]
/*
enum IpAddress {
    V4(String),
    V6(String),
}
pub fn enums() {
    let home = IpAddress::V4("127.0.0.1".to_string());
    println!("{:?}", home);
}
*/
enum Human {
    Single(bool, bool),
    Mingle(bool, bool),
}
pub fn enums() {
    let human1 = Human::Single(true, true);
    println!("{:?}", human1);
    let human2 = Human::Single(true, true);
    println!("{:?}", human2);
}

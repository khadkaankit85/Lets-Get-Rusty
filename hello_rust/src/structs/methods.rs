#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
impl Rectangle {
    fn get_area(&self) -> u32 {
        let area = self.width * self.height;
        area
    }
    fn print_side_count() {
        println!("I have 4 sides");
    }
    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        let can_hold = self.get_area() > another_rect.get_area();
        can_hold
    }
    fn create_square(side: u32) -> Self {
        let new_rect = Rectangle {
            width: side,
            height: side,
        };
        new_rect
    }
}

#[allow(dead_code)]
pub fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 80,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 80,
    };
    println!(" the width of Rectangle  is {:#?}", rect1.get_area());
    Rectangle::print_side_count();
    println!(" rect1 can hold hold rect2 :{} ", rect2.can_hold(&rect1));

    let square1 = Rectangle::create_square(70);
    println!(" square1 {:#?}", square1);
}

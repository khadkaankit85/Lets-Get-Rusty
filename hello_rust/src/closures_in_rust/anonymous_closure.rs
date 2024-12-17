#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn anonymous() {
    let mut rectanges = [
        Rectangle {
            width: 10,
            height: 10,
        },
        Rectangle {
            width: 5,
            height: 5,
        },
        Rectangle {
            width: 20,
            height: 20,
        },
    ];

    let mut sort_count = vec![];

    rectanges.sort_by_key(|r| {
        sort_count.push(1);
        r.width
    });
    println!("sorted by width rectangles are : {rectanges:?} and called the function sort count {sort_count:?} times");
}

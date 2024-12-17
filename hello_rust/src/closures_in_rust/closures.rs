#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum ShirtColor {
    Red,
    Blue,
}
#[allow(dead_code)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[allow(dead_code)]
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }
        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[allow(dead_code)]
pub fn closures() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
}

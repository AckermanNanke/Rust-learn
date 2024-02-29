#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventtory {
    shirts: Vec<ShirtColor>,
}

impl Inventtory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for i in &self.shirts {
            match i {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn run_closures_demo1() {
    let storeShirts = Inventtory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Red],
    };
    let user_preference1: Option<ShirtColor> = Some(ShirtColor::Blue);
    let giveaway1 = storeShirts.giveaway(user_preference1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference1, giveaway1
    );
    let user_preference2: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway2 = storeShirts.giveaway(user_preference1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference2, giveaway2
    );
}

pub fn run_closures_demo() {
    run_closures_demo1()
}

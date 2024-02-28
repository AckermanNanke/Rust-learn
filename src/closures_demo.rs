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
                Red => num_red += 1,
                Blue => num_blue += 1,
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
    storeShirts
}

pub fn run_closures_demo() {
    run_closures_demo1()
}

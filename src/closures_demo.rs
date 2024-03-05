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
    let store_shirts = Inventtory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_preference1: Option<ShirtColor> = Some(ShirtColor::Blue);
    let giveaway1 = store_shirts.giveaway(user_preference1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference1, giveaway1
    );
    let user_preference2: Option<ShirtColor> = None;
    let giveaway2 = store_shirts.giveaway(user_preference2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference2, giveaway2
    );
}

fn run_closures_demo2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
pub fn run_closures_demo() {
    run_closures_demo1();
    run_closures_demo2();
}

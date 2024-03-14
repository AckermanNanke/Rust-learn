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

// 3种闭包
// FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
// FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。
// Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。
fn run_closures_demo2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    borrows_mutably();
    println!("可变闭包引用值 {:?}", list);

    let mut list1 = vec![String::from("1"), String::from("2")];
    let v1 = String::from("3");
    let borrows_mutably1 = || list1.push(v1);
    borrows_mutably1();
    // borrows_mutably1();
    println!("可变闭包引用值 {:?}", list1);
}
pub fn run_closures_demo() {
    run_closures_demo1();
    run_closures_demo2();
}

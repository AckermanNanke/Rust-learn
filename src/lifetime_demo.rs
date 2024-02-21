// 错误示范
// fn run_lifetime_demo1() {
//     let v;
//     {
//         let s = 18;
//         v = &s;
//     }
//     println!("run_lifetime_demo1 = {} \n", v)
// }
// fn get_str(x: &str, y: &str) -> &str {
//     // let c = String::from("789");
//     // c.as_str()
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

struct LifetimeStruct<'a> {
    a: &'a str,
}

// 生命周期注解
fn run_lifetime_demo2() {
    let a = "123";
    let b = "456";
    let c = "789";
    println!("run_lifetime_demo2 = {}", get_str(a, b, c))
}
fn get_str<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str
where
    T: Display,
{
    println!("run_lifetime_demo3.z = {}", z);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run_lifetime_demo() {
    run_lifetime_demo2()
}

use crate::boxt_demo::List::{Cons, Nil};
use std::ops::Deref;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 实现deref强制转换
struct DemoBox<T>(T);

impl<T> Deref for DemoBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DemoBox<T> {
    fn new(x: T) -> DemoBox<T> {
        DemoBox(x)
    }
}

// 实现deref强制转换测试
fn run_boxt_demo2() {
    let x = 5;
    let y = DemoBox::new(x);
    let z = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

// 堆数据
fn run_boxt_demo1() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list)
}

pub fn run_boxt_demo() {
    // run_boxt_demo1();
    run_boxt_demo2();
}

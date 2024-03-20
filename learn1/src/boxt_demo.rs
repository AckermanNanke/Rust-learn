use crate::boxt_demo::List::{Cons, Nil};
use std::ops::Deref;
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 实现deref强制转换
struct DemoBox<T>(T);

// 实现销毁数据
#[derive(Debug)]
struct DropBox {
    data: String,
}

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

impl Drop for DropBox {
    fn drop(&mut self) {
        println!("Dropping DropBox with data `{:#?}`!", self.data);
    }
}

// 实现deref强制转换测试
fn run_boxt_demo_deref() {
    let x = String::from("5");
    let y = DemoBox::new(x);
    let z = &y;

    assert_eq!("5", *y);
}
fn run_boxt_demo_drop() {
    let c = DropBox {
        data: String::from("my stuff"),
    };
    let d: DropBox = DropBox {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("Dropping DropBox with data `{:#?}`!", c);
    println!("Dropping DropBox with data `{:#?}`!", d);
}

// 堆数据
fn run_boxt_demo1() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list)
}

pub fn run_boxt_demo() {
    run_boxt_demo1();
    run_boxt_demo_deref();
    run_boxt_demo_drop();
}

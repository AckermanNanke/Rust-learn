use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::rct_demo::List::{Cons, Nil};
// Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a。接着当再次尝试使用 a 创建 c 时，这不被允许，因为 a 的所有权已经被移动。
// 可以改变 Cons 的定义来存放一个引用，不过接着必须指定生命周期参数。通过指定生命周期参数，表明列表中的每一个元素都至少与列表本身存在的一样久。这是示例 15-17 中元素与列表的情况，但并不是所有情况都如此。
// enum List<'a> {
//     Cons(i32, Box<&'a List<'a>>),
//     Nil,
// }

// fn rct_demo1() {
//     let binding = (Cons(10, Box::new(&Nil)));
//     let a = Cons(5, Box::new(&binding));
//     let b = Cons(3, Box::new(&a));
//     let c = Cons(4, Box::new(&a));
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// // 需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。在 main 中创建了存放 5 和 10 的列表并将其存放在 a 的新的 Rc<List> 中。接着当创建 b 和 c 时，调用 Rc::clone 函数并传递 a 中 Rc<List> 的引用作为参数。
// // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
// fn run_rct_demo1() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 树节点
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn run_rct_demo1() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a 引用计数 = {}", Rc::strong_count(&a));
    println!("a 下一项 = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    //取消下一行的注释，将溢出堆栈，因为引用循环
    // println!("a next item = {:?}", a.tail());
}

fn run_rct_demo2() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
pub fn run_rct_demo() {
    run_rct_demo1();
}

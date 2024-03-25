use std::rc::Rc;

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

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// 需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。在 main 中创建了存放 5 和 10 的列表并将其存放在 a 的新的 Rc<List> 中。接着当创建 b 和 c 时，调用 Rc::clone 函数并传递 a 中 Rc<List> 的引用作为参数。
// 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
fn run_rct_demo1() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn run_rct_demo2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

trait Msger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Msger> {
    msger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Msger,
{
    fn new(msger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            msger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let max = self.value as f64 / self.max as f64;
        print!("max = {}", max);
        if max >= 1.0 {
            self.msger.send("Error: You are over your quota!");
        } else if max >= 0.9 {
            self.msger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if max >= 0.75 {
            self.msger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMsger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMsger {
        fn new() -> MockMsger {
            MockMsger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Msger for MockMsger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_msger = MockMsger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_msger.sent_messages.borrow().len(), 1);
    }
}

pub fn run_rct_demo() {
    run_rct_demo1();
    run_rct_demo2();
}

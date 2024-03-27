#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // 可变智能指针RefCell<T>测试
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

    #[derive(Debug)]
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
            // // 存在多个可变借用，编译命令不会报错，测试命令会报错
            let mut s1 = self.sent_messages.borrow_mut();
            s1.push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_msger = MockMsger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msger, 100);
        limit_tracker.set_value(80);

        println!("mock_msger =========================== {:#?}", mock_msger);
        assert_eq!(mock_msger.sent_messages.borrow().len(), 1);
    }
}

// 多线程处理

use std::thread;
use std::time::Duration;

fn run_thread_demo1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("i = {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for x in 1..5 {
        println!("x = {}", x);
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn run_thread_demo() {
    run_thread_demo1()
}

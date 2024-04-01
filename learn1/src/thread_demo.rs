// 多线程处理

use std::{sync::mpsc, thread, time::Duration};

fn run_thread_demo3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("he"), String::from("hr")];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi1"),
            String::from("he1"),
            String::from("hr1"),
        ];
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {:#?}", received);
    }
}

fn run_thread_demo2() {
    let v = vec![1, 2, 3];
    let h = thread::spawn(move || {
        println!("v = {:?}", v);
    });
    // drop(v);
    h.join().unwrap();
}

fn run_thread_demo1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i = {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for x in 1..5 {
        println!("x = {}", x);
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn run_thread_demo() {
    // run_thread_demo1();
    // run_thread_demo2();
    run_thread_demo3();
}

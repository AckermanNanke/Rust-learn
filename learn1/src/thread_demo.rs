// 多线程处理

use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

// 互斥锁
fn run_thread_mutex() {
    let mut counter = Arc::new(Mutex::new(0));
    let mut hadles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("num = {:?}", num);
        });
        hadles.push(handle);
    }

    for handle in hadles {
        handle.join().unwrap();
    }
    println!("counter = {:?}", *counter.lock().unwrap());
}

// 互斥锁死锁
// fn run_thread_mutex_error() {
//     let counter1 = Arc::new(Mutex::new(0));
//     let counter2 = Arc::new(Mutex::new(0));
//     let c1 = Arc::clone(&counter1);

//     let mut h1 = vec![];

//     let hl1 = thread::spawn(move || {
//         let num1 = counter1.lock().unwrap();
//         println!("hl1.num1 = {:?}", num1);
//         let num2 = c1.lock().unwrap();
//         println!("hl1.num2 = {:?}", num2);
//     });
//     h1.push(hl1);

//     for handle in h1 {
//         handle.join().unwrap();
//     }
// }
fn run_thread_mutex_error() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let c1 = Arc::clone(&counter2);
    let c2 = Arc::clone(&counter1);

    let mut h1 = vec![];
    let mut h2 = vec![];

    let hl1 = thread::spawn(move || {
        let num1 = counter1.lock().unwrap();
        println!("hl1.num1 = {:?}", num1);
        let num2 = c2.lock().unwrap();
        println!("hl1.num2 = {:?}", num2);
    });
    h1.push(hl1);

    let hl2 = thread::spawn(move || {
        let num2 = c1.lock().unwrap();
        let num1 = counter2.lock().unwrap();
        println!("hl2.num2 = {:?}", num2);
        println!("hl2.num1 = {:?}", num1);
    });
    h2.push(hl2);

    for handle in h1 {
        handle.join().unwrap();
    }
    for handle in h2 {
        handle.join().unwrap();
    }
}

// 信道
fn run_thread_channel() {
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
    // run_thread_channel();
    // run_thread_mutex();
    run_thread_mutex_error();
}

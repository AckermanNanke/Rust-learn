use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct learnmodStruct {
    name: String,
    age: i32,
    sex: bool,
}

fn vecDemo() {
    // Vec可变列表
    let mut v = vec![1];
    let v1 = v[0];
    v.push(2);
    v[0] = 10;
    print!("{:#?} \n", v);
    print!("{:#?} \n", v1);
    print!("{:#?} \n", &v[0]);
}

fn strDemo() {
    // 字符串
    let mut s1: String = String::from("s1");
    let s2: String = String::from("s1");
    let s3: String = "initial contents".to_string();
    let s4: &str = "s2";
    let str1 = s1 + &s2;

    use std::collections::HashMap;
}

fn hashMapDemo() {
    // 哈希键值对，同质
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // print!("{:#?} \n", field_name);
    let mut hmap = HashMap::new();
    hmap.insert(1, 001);
    hmap.entry(1).or_insert(003);
    let count = hmap.entry(1).or_insert(003);
    *count += 1;
    print!("count={:#?} \n", count);
    print!("hmap={:#?} \n", hmap);
}

// 错误处理
fn errDemo1(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn errDemo2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn runLearnDemo() {
    let learnmodstruct = learnmodStruct {
        name: String::from("learn"),
        age: 13,
        sex: true,
    };
    print!("{:#?}", learnmodstruct);
}

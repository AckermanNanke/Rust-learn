// trait行为学习
struct Trait1Struct1 {
    name: String,
    age: i32,
}

struct Trait1Struct2 {
    name: String,
    age: i32,
    sex: bool,
}
#[derive(Debug)]
struct TraitStruct3<Q, W> {
    name: Q,
    age: W,
}

// 普通
trait Trait1 {
    fn get_name(&self) -> String;
    fn get_age(&self) -> i32 {
        99
    }
}
pub trait Trait2 {
    fn get_sex(&self) -> bool;
}

// 泛型
trait Trait3<Q, W> {
    fn get_name(&self) -> &Q;
    fn get_age(&self) -> &W;
}

// 普通实现
impl Trait1 for Trait1Struct1 {
    fn get_name(&self) -> String {
        format!("@{}", self.name)
    }
}
impl Trait1 for Trait1Struct2 {
    fn get_name(&self) -> String {
        format!("@{}", self.name)
    }
}
impl Trait2 for Trait1Struct2 {
    fn get_sex(&self) -> bool {
        self.sex
    }
}

// 泛型实现
impl<Q, W> Trait3<Q, W> for TraitStruct3<Q, W> {
    fn get_name(&self) -> &Q {
        &self.name
    }
    fn get_age(&self) -> &W {
        &self.age
    }
}

// 普通trait
fn run_trait_demo1() {
    let demo: Trait1Struct1 = Trait1Struct1 {
        name: String::from("myw1"),
        age: 11,
    };
    println!("run_trait_demo1.name = {}", demo.get_name());
    println!("run_trait_demo1.age = {}", demo.get_age());
}

// 参数trait
// 限制同类型
fn run_trait_demo2<T: Trait1>(item1: &T, item2: &T) {
    println!("run_trait_demo2-item1.name = {}", item1.get_name());
    println!("run_trait_demo2-item2.name = {}", item2.get_name());
}
// 不限制同类型
fn run_trait_demo3(item1: &impl Trait1, item2: &impl Trait1) {
    println!("run_trait_demo3-item1.name = {}", item1.get_name());
    println!("run_trait_demo3-item2.name = {}", item2.get_name());
}

// 同时实现多个trait
fn run_trait_demo4<T>(item: &T)
where
    T: Trait1 + Trait2,
{
    println!("run_trait_demo4.sex = {}", item.get_sex());
    // println!("run_trait_demo4.name = {}", item.get_name());
}
// 同时实现多个trait
fn run_trait_demo5<Q, W, T: Trait3<Q, W>>(item: &T) {
    item.get_name();
    // println!("run_trait_demo5.name = {}", x);
}

// 返回实现trait
pub fn run_trait_demo() -> impl Trait2 {
    let trait_struct1 = Trait1Struct1 {
        name: String::from("myw1"),
        age: 11,
    };
    let trait_struct2: Trait1Struct2 = Trait1Struct2 {
        name: String::from("myw2"),
        age: 12,
        sex: false,
    };
    let trait_struct3 = TraitStruct3 {
        name: "myw3",
        age: 13,
    };
    run_trait_demo1();
    run_trait_demo2(&trait_struct1, &trait_struct1);
    run_trait_demo3(&trait_struct1, &trait_struct2);
    run_trait_demo4(&trait_struct2);
    run_trait_demo5(&trait_struct3);
    trait_struct2
}

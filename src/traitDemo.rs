// trait行为学习
struct TraitStruct1 {
    name: String,
    age: i32,
}

struct TraitStruct2<Q, W> {
    name: Q,
    age: W,
}
struct TraitStruct3<Q, W> {
    name: Q,
    age: W,
}

trait Trait1 {
    fn getName(&self) -> String;
    fn getAge(&self) -> i32 {
        99
    }
}

impl Trait1 for TraitStruct1 {
    fn getName(&self) -> String {
        format!("@{}", self.name)
    }
}

impl<Q, W> Trait1 for TraitStruct3<Q, W> {
    fn getName(&self) -> String {
        format!("@{}", self.name)
    }
}

// 普通trait
fn runTraitDemo1() {
    let demo = TraitStruct1 {
        name: String::from("myw1"),
        age: 11,
    };
    println!("runTraitDemo.name = {}", demo.getName());
    println!("runTraitDemo.age = {}", demo.getAge());
}

// 参数trait
fn runTraitDemo2(item: &impl Trait1) {
    println!("runTraitDemo.name = {}", item.getName());
}
fn runTraitDemo3<T: Trait1>(item1: &T, item2: &T) {
    println!("runTraitDemo.name = {}", item1.getName());
    println!("runTraitDemo.name = {}", item2.getName());
}

pub fn run_trait_demo() {
    let trait_struct1 = TraitStruct1 {
        name: String::from("myw1"),
        age: 11,
    };
    let trait_struct3 = TraitStruct3 {
        name: "myw3",
        age: 13,
    };
    runTraitDemo1();
    runTraitDemo2(&trait_struct1);
    runTraitDemo3(&trait_struct1, &trait_struct3);
}

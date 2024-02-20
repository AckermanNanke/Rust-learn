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
struct TraitStruct3<Q, W> {
    name: Q,
    age: W,
}

// 普通
trait Trait1 {
    fn getName(&self) -> String;
    fn getAge(&self) -> i32 {
        99
    }
}
pub trait Trait2 {
    fn getSex(&self) -> bool;
}

// 泛型
trait Trait3<Q, W> {
    fn getName(&self) -> Q;
    fn getAge(&self) -> i32 {
        99
    }
}

// 普通实现
impl Trait1 for Trait1Struct1 {
    fn getName(&self) -> String {
        format!("@{}", self.name)
    }
}
impl Trait1 for Trait1Struct2 {
    fn getName(&self) -> String {
        format!("@{}", self.name)
    }
}
impl Trait2 for Trait1Struct2 {
    fn getSex(&self) -> bool {
        self.sex
    }
}

// 泛型实现
// impl<Q, W> Trait3<Q, W> for TraitStruct3<Q, W> {
//     fn getName(&self) -> Q {
//         self.name
//     }
// }

// 普通trait
fn runTraitDemo1() {
    let demo: Trait1Struct1 = Trait1Struct1 {
        name: String::from("myw1"),
        age: 11,
    };
    println!("runTraitDemo1.name = {}", demo.getName());
    println!("runTraitDemo1.age = {}", demo.getAge());
}

// 参数trait
// 限制同类型
fn runTraitDemo2<T: Trait1>(item1: &T, item2: &T) {
    println!("runTraitDemo2-item1.name = {}", item1.getName());
    println!("runTraitDemo2-item2.name = {}", item2.getName());
}
// 不限制同类型
fn runTraitDemo3(item1: &impl Trait1, item2: &impl Trait1) {
    println!("runTraitDemo3-item1.name = {}", item1.getName());
    println!("runTraitDemo3-item2.name = {}", item2.getName());
}

// 同时实现多个trait
fn runTraitDemo4<T: Trait1 + Trait2>(item: &T)
where
    T: Trait1 + Trait2,
{
    println!("runTraitDemo4.sex = {}", item.getSex());
    // println!("runTraitDemo4.name = {}", item.getName());
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
    runTraitDemo1();
    runTraitDemo2(&trait_struct1, &trait_struct1);
    runTraitDemo3(&trait_struct1, &trait_struct2);
    runTraitDemo4(&trait_struct2);
    trait_struct2
}

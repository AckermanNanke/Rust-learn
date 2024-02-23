#[derive(Debug)]
struct GenericsStruct<Q, W, E> {
    name: Q,
    age: W,
    sex: E,
}

impl<Q, W, E> GenericsStruct<Q, W, E> {
    fn get_generics_struct_name(self, other: GenericsStruct<Q, W, E>) -> GenericsStruct<Q, W, E> {
        GenericsStruct {
            name: self.name,
            age: self.age,
            sex: other.sex,
        }
    }
}

fn generics_demo1<T>(list: &[T]) -> &T {
    let mut largst = &list[0];
    for i in list {
        // if i > list {
        largst = i
        // }
    }
    largst
}
fn generics_demo2() {
    let mut genericsstruct = GenericsStruct {
        name: "myw",
        age: 12,
        sex: false,
    };
    print!("genericsstruct = {:#?}", genericsstruct);
}

pub fn run_generics_demo() {
    let list = vec![23, 45, 34, 4];
    generics_demo1(&list);
    generics_demo2();
}

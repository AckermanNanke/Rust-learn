#[derive(Debug)]
struct GenericsStruct<Q, W, E> {
    name: Q,
    age: W,
    sex: E,
}

impl<Q, W, E> GenericsStruct<Q, W, E> {
    fn getGenericsStructName(self, other: GenericsStruct<Q, W, E>) -> GenericsStruct<Q, W, E> {
        GenericsStruct {
            name: self.name,
            age: self.age,
            sex: other.sex,
        }
    }
}

fn genericsDemo1<T>(list: &[T]) -> &T {
    let mut largst = &list[0];
    for i in list {
        // if i > list {
        largst = i
        // }
    }
    largst
}
fn genericsDemo2() {
    let mut genericsstruct = GenericsStruct {
        name: "myw",
        age: 12,
        sex: false,
    };
    print!("genericsstruct = {:#?}", genericsstruct);
}

pub fn runGenericsDemo() {
    let list = vec![23, 45, 34, 4];
    genericsDemo1(&list);
    genericsDemo2();
}

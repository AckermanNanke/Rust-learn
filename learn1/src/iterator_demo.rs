// 迭代器
pub fn run_iterator_demo() {
    let v1 = vec![1, 2, 3];
    let mut i1 = v1.iter();
    assert_eq!(i1.next(), Some(&1));
    assert_eq!(i1.next(), Some(&1));
}

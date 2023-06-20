use vec::MyVec;

fn main() {
    let mut vec: MyVec<usize> = MyVec::new();
    vec.push(1usize);
    vec.push(2);
    vec.push(3);
    vec.push(3);
    vec.push(3);

    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.get(2), Some(&3));
    assert_eq!(vec.len(), 5);
}

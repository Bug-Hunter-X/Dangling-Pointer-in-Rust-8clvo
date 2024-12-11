fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of a raw pointer, use a reference.
    let first_element = &mut v[0];
    *first_element = 10;
    println!("{:?}", v);
} 
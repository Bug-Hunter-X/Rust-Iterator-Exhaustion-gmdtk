fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let iter = vec.iter();
    let mut iter2 = vec.iter();
    println!( "{}", iter.next().unwrap());
    println!( "{}", iter2.next().unwrap());
    // Iterators are independent of each other.
}
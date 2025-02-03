fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!( "{}", iter.next().unwrap());
    println!( "{}", iter.next().unwrap());
    //println!( "{}", iter.next().unwrap()); // This line will cause a panic if uncommented
}
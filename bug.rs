fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1];
    println!("The second element is: {}", second_element);
    // This line will cause a panic because the index is out of bounds.
    let fourth_element = vec[3];
    println!("The fourth element is: {}", fourth_element);
}
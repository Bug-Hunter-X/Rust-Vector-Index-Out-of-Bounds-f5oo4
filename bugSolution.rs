fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1];
    println!("The second element is: {}", second_element);
    // Check if the index is within bounds before accessing the element
    if vec.len() > 3 {
        let fourth_element = vec[3];
        println!("The fourth element is: {}", fourth_element);
    } else {
        println!("The vector does not have a fourth element.");
    }
} 
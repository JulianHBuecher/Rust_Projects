fn main() {
    // s is not initialized
    
    // Requesting memory for this method
    let mut s = String::from("hello");
    // s is in scope

    s.push_str(", world");

    println!("{}", s);

} // s goes out of scope
// so the allocated memory for the String will be returned

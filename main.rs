fn main() {
    let s : String = String::from("hcvk");
    let r: &str = &s;
    let s2 : &str = String::from("hcvk1").as_str();
    println!("{}", s); // Prints: Hello, world!
    println!("{}", r); // Prints: Hello, world!
    println!("{}", s2); // Prints: Hello, world!
}
fn main() {
    let mut x=5; // variable x is immutable by default, we can make it mutable by adding the keyword `mut` 
    println!("The value of x is {x}");
    x=10;
    println!("The renewed value is {x}.");
}

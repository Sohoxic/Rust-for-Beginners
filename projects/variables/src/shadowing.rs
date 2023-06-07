fn main() {
    let x = 1;
    let x = x+1;
    {
        let x=x*2;
        println!("The value of x inside the inner scope is {x}");
    }
    println!("The value of x outside the scope is {x}");
}

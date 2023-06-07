fn main() {
    let x = 1;
    let x = x+1;
    {
        let x=x*2;
        println!("The value of x inside the inner scope is {x}");
    }
    println!("The value of x outside the scope is {x}");
}

fn main() {
    let mut x: u32 = 1;
    {
      let mut x = x;
      x += 2;
    }
    println!("{x}");
  }

 // The below code won't run as because we're trying to change the type of the mutable variable without shadowing. 
  fn main() {
    let mut x: u32 = 1;
    x = "Hello world";
    println!("{x}");
  }
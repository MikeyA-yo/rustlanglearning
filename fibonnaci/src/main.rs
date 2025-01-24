fn main() {
    println!("Hello, world!");
    let no = 7;
    let x = generate_fibonnaci(no);
    println!("Fibonnaci of {no} is {x}");
}

fn generate_fibonnaci(x: u32) -> u32{
  if x == 1 {
    0
  } else if x == 2  {
    1
  } else {
    // prev last no of x + last no of x
   generate_fibonnaci(x-2) + generate_fibonnaci(x-1)
  }
}
fn main() {
    println!("Hello, world!");
    let no = 5;
    let x = generate_fibonnaci(no);
    println!("Fibonnaci of {no} is {x}");
}

fn generate_fibonnaci(x: u32) -> u32{
  if x == 0 {
    0
  } else if x == 1 {
    1
  } else {
    x + generate_fibonnaci(x-1)
  }
}
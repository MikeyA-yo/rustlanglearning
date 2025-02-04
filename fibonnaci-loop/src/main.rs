use std::io;
fn main() {
    println!("Please input an index to see the Fibonnaci position!");
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect("Failed to read input");
    let no:usize = no.trim().parse().expect("Expect a valid number");
    println!("Generating fibonnaci of index positon: {no}.....");
    let x = generate_fibonnaci(no);
    println!("Fibonnaci of {no} is {x}");

}

fn generate_fibonnaci(x: usize) -> usize {
    if x == 1 {
        return 0;
    } else if x == 2 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..x {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}
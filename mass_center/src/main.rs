fn main() {
    println!("Hello, world!");
    let center_of_mass = center_mass(4, 3, 4, 12);
    println!("The center of mass is {center_of_mass}");
}

fn center_mass(m1:i32, x1:i32, m2:i32, x2:i32) -> f32 {
 let total_mass = m1 + m2;
 let first_product =  m1 * x1;
 let second_product = m2 * x2;
 let mass_center =(first_product + second_product) as f32 / total_mass as f32 ;
 mass_center 
}
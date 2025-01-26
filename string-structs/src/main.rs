fn main() {
    let mut name = String::from("Ayomide Oluwatola");
    let user = string_ref(&mut name);
    println!("{user}");
}

fn string_ref(s: &mut String) -> &String{
    s
}
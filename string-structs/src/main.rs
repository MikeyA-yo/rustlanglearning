#[derive(Debug)]
struct User {
    name : String,
    age: u16
}
fn main() {
    let name = String::from("Ayomide Oluwatola");
    let age = 17;
    let user = new_user(name, age);
    println!("The new user is: {user:?}");
    let user2 = new_user(String::from("Rapto"), user.age);
    println!("The last user is: {user2:?}");
    let subname = &user.name[0..user.name.len()/2];
    println!("{}", subname);
}


fn new_user(name:String, age:u16) -> User {
  User {
    name,
    age
  }
}
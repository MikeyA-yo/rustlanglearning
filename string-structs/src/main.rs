#[derive(Debug)]
struct User {
    name : String,
    age: u16
}

impl User {
  fn format(&self) -> String{
    let age_str = &self.age.to_string();
    let  fmt = self.name.clone()+ &String::from(" ") + age_str;
    fmt
  }
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
    println!("User format: {}", user.format());
    println!("User format: {}", user2.format());
}


fn new_user(name:String, age:u16) -> User {
  User {
    name,
    age
  }
}
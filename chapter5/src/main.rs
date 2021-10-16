fn main() {
  let user1 = User {
    username: String::from("DineshKumar"),
    email: String::from("criticalopsnoob1302@gmail.com"),
    active: true,
  };
  println!("The Name of the user: {}\nThe Email of the user: {}\nIs the user active: {}"
           , user1.username, user1.email, user1.active);
  let place1 = Rectangle {
    width: 10,
    length: 10,
  };
  let place2 = Rectangle {
    length: 2,
    width: 3,
  };
  println!("The User1 is at place '{}:{}', with area: {}", place1.width, place1.length, place1.area());
  println!("{}", place1.can_hold(&place2));

}

struct User {
  username: String,
  email: String,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  length: u32
}

// fn area(bruh: &Area) -> i32 {
//   bruh.width * bruh.length
// }

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.length
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    (self.length > other.length) && (self.width > self.width)
  }
}

fn main() {
  s1 = gives_ownership();
  println!("{}", s1);
}

fn gives_ownership() -> String {
  let some_string = String::from("Hello");
  some_string
}

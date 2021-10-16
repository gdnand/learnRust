fn main() {
  let s1 = grant_ownership();
  let s2 = String::from("hello");
  let s3 = take_a_string(s2);
  println!("{}", s2);

}

fn grant_ownership() -> String {
  let s = String::from("hello");
  s
}

fn take_a_string(a_string: String) -> String {
  a_string
}


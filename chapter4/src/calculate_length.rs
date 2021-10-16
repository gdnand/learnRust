fn main() {
  let s1 = String::from("Hello");
  let len = calculate_length(&s1);
  println!("The Length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> (String, usize) { // Gets a reference from s1
  let length = s.len();
  (s, length)
}

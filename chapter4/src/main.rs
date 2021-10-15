fn main() {
  // ownership rules!
  let bruh = {
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.len()
  };
  let s1 = String::from("hello!");
  let s2 = s1; // Moved ownership to s2.
  // println!("{}", s1); // Doesn't work.
  println!("{}", &s2);
  println!("{0}",  bruh);
}

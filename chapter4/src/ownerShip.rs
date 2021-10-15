fn main() {
  let x = String::from("Dinesh");
  bruh(x);
  println!("{}", x); // Hasn't implemented copy trait, therefore can't 
                    //  come into scope again.
  let x: i32 = 5;
  bruh1(x);
  println!("{}", x);
}

fn bruh(x: String) {
  println!("{}", x);
}

fn bruh1(x: i32) {
  println!("{}", x);
}

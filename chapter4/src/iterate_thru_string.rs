fn main() {
  let x = String::from("hello the best word");
  println!("The Length of the first word in '{}' is {}", x, first_word(&x));
  println!("{:?}", second_word(&x));
  let (y, z) = second_word(&x);
  println!("{}", &x[y..z]);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }
  s.len()
}

fn second_word(s: &String) -> (usize, usize) {
  let bytes = s.as_bytes();
  let starting_point = first_word(&s) + 1;
  for (i, &item) in bytes.iter().enumerate() {
    if i < starting_point {
      continue;
    } else {
      if item == b' ' {
        return (starting_point, i);
      }
    }
  }
  (0, s.len())
}

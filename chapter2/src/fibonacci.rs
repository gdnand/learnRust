fn main() {
  for number in 1..8 {
    println!("{}", fibonacci(number));
  }
}

fn fibonacci(nth: i32) -> i32 {
  if nth <= 1 {
    return 1
  } else {
    return fibonacci(nth - 1) + fibonacci(nth - 2)
  }
}

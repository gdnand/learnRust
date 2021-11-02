fn main() {
  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  let sum = x + y.unwrap(); // I want to unwrap this some type.
  println!("{}", sum);
  let five = Some(5);
  let six = plus_one(five);
  let _none = plus_one(None);
  println!("{:?}", six.unwrap());
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1),
  }
}

fn main() {
    let x = 45;
    println!("The Value of x is: {}", x);
    let x = x + 1;
    println!("The Value of x is: {}", x);
    let x = x * 2;
    println!("The Value of x is: {}", x);

    let guess: i32 = "42".parse().expect("Not a number!");
    println!("The Value of guess is: {}", guess);
    let _f: bool = true;

    let array1 = [1, 2, 3, 4, 5];
    let first = array1[0];
    println!("{}", &first);
    let dinesh = another_function(10, 12);
    println!("{:?}", &dinesh);
    let dinesh = five();
    println!("{:?}", &dinesh);
    let another_var = assign_value(10, 12);
    println!("{}", another_var);
}

fn another_function(x: i32, y: i32) -> (i32, i32) {
  println!("Another Function is called.");
  println!("The Value of x is: {}", x);
  println!("The Value of y is: {}", y);
  (x, y)
}

fn five() -> i32 {
  5
}

fn assign_value(x: i32, y: i32) -> i32 {
  let z = if x > y {
    x
  } else {
    y
  };
  z
}

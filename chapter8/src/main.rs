use hashmaps;

fn main() {
  // let v: Vec<i32> = Vec::new(); // One Way to do this
  // let v = vec![1, 2, 3];
  // let a = [1, 2, 3];
  // println!("{:?}", v);
  // assert_eq!(v, a);
  let mut v: Vec<i32> = Vec::new();
  v.push(4);
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);
  println!("{:?}", v);
  let mut vector = vec![1, 2, 3, 4, 5];
  // let third: Option<&i32> = v.get(2);
  let _third: Option<bool> = None;
  // println!("{:?}", third.unwrap()); // can't unwrap a None
  for items in &mut vector {
    *items += 50;
    println!("{}", items);
  }
  println!("{:?}", vector);
  let s :String = String::from("Initial Contents");
  let v: String = String::from(" Initial Contents 2");
  println!("{}", s);
  let b: String = s[0..4].to_string();
  println!("{}", s + &v);
  println!("{}", b);
}

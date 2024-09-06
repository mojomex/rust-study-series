fn add_one(i: i32) {
  i += 1
}

pub fn basics() {
  let i = 5;
  add_one(i);
  println!("{}", i);
}
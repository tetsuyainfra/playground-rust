fn main() {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;

  println!("Before mutation: {}", mutable_binding);

  // Ok
  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);

  // Error!
  // _immutable_binding += 1; // let だけなので不変であると示されてる
  // FIXME ^ Comment out this line
  // FIXME ^ この行をコメントアウトしましょう
}
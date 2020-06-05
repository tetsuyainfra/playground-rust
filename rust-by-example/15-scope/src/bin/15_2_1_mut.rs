// mutability 変化できる性質
// mutability は 所有権を移すときに変更できる

fn main() {
  let immutable_box = Box::new(5u32);
  println!("immutable_box contains {}", immutable_box);

  // 不変な変数に所有権があるため 変更できない
  // *immutable_box = 100u32; // error

  // move と同時に所有権とミュータビリティを変更する
  let mut mutable_box = immutable_box;
  println!("mutable_box contains {}", mutable_box);
  *mutable_box = 100u32; // OK

  println!("mutable_box now contains {}", mutable_box);
}

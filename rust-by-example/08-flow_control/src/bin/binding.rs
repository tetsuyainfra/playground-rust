fn age() -> u32 {
  15
}

fn main() {
  println!("Tell me what type of person you are");

  match age() {
    0 => println!("I'm not born yet I guess"),
    // `1 ... 12`の値を一挙に`match`させることができる。
    // しかしその場合、子供は正確には何歳?
    // マッチした値を`n`にバインディングすることで値を使用できる。 <---
    n @ 1..=12 => println!("I'm a child of age {:?}", n),
    n @ 13..=19 => println!("I'm a teen of age {:?}", n), //<---
    // Nothing bound. Return the result.
    // マッチしなかった場合の処理
    n => println!("I'm an old person of age {:?}", n),
  }

  // enumでもできる
  fn some_number() -> Option<u32> {
    Some(42)
  }

  match some_number() {
    Some(n @ 42) => println!("The answer is {}", n),
    Some(n) => println!("Not Interesting ... {}", n),
    _ => (),
  }
}

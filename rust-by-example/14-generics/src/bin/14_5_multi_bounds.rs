// フクスウノジェネリック境界
// +を用いて複数のトレイト境界を用いることができる
// 複数の引数を受け取るときは,で区切る

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: `{:?}`", t);
  println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
  println!("t: `{:?}`", t);
  println!("u: `{:?}`", u);
}

// 追加
// これだと引数が同じ型じゃないとダメになる
fn compare_types2<T: Debug>(t: &T, u: &T) {
  println!("t: `{:?}`", t);
  println!("u: `{:?}`", u);
}

fn main() {
  let string = "words";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  // compare_prints(&array);
  // TODO ^ Try uncommenting this.
  // TODO ^ ここをアンコメントしてみましょう。

  compare_types(&array, &vec);
}

// || は()で入力変数の代わりにできる
// 式が単一だったら{}を使わなくてよい
// スコープ外の変数をキャプチャーできる

fn main() {
  // 関数とクロージャのそれぞれで数値をインクリメントする
  fn function(i: i32) -> i32 {
    i + 1
  }

  // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
  // `{}`も必須ではない。
  // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i| i + 1;

  let i = 1;
  // 関数とクロージャを呼び出す。
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));
  // println!("closure_inferred2: {}", closure_inferred(32u32)); // error -> closure内で推定inferencingしている + 1 のところ

  // A closure taking no arguments which returns an `i32`.
  // The return type is inferred.
  let one = || 1;
  println!("closure returning one: {}", one());
}

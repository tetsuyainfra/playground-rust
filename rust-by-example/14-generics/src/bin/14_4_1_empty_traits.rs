// testcase: empty trait
// トレイト境界の仕組みから
// トレイト境界が何も機能を持っていなくとも境界条件として使用できる
// という帰結がもたらされる
// Eq, Ordはstdライブラリでの使用例

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を設けているが
// トレイトが空でも問題ない
fn red<T: Red>(_: &T) -> &'static str {
  "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
  "blue"
}

fn main() {
  // 猩々紅冠鳥 ショウジョウコウカンチョウ
  let cardinal = Cardinal;
  // アオカケス
  let blue_jay = BlueJay;
  // 七面鳥
  let _turkey = Turkey;

  // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
  // `blue`と`Cardinal`も同様、
  println!("A cardinal is {}", red(&cardinal));
  println!("A blue jay is {}", blue(&blue_jay));
  // println!("A turkey is {}", red(&_turkey));
  // ^ TODO: Try uncommenting this line.
  // ^ TODO: この行をアンコメントしてみましょう。
}

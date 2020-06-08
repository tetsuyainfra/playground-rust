// 継承 Derive
// コンパイラには[#derive]アトリビュートを使って型に対して
// 特定の標準的な実装を提供する機能がある
// より複雑なことを行わせたいときは同名のトレイトを手動で実装することが可能

// derive可能なトレイト一覧
// - Eq, PartialEq, Ord, PartialOrd : 型の比較に関するトレイト
// - Clone : コピーによって&TからTを作成するトレイト
// - Copy : Moveセマンティクスの代わりにコピーセマンティクスを提供する
// - Hash : &Tからハッシュ値を計算するためのトレイト
// - Default : 空のインスタンスを作成するためのトレイト（初期化が楽になる）
// - Debug : {:?}フォーマッタを利用して値をフォーマットするためのトレイト

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
  fn to_centimeters(&self) -> Centimeters {
    let &Inches(inches) = self;
    Centimeters(inches as f64 * 2.54)
  }
}

struct Seconds(i32);

fn main() {
  let _one_second = Seconds(1);

  // エラー: `Seconds`はプリントできない。これは`Debug`トレイトを実装していないため
  //println!("One second looks like: {:?}", _one_second);

  // エラー: `Seconds`は比較できない。これは`PartialEq`トレイトを実装していないため
  //let _this_is_true = (_one_second == _one_second);

  let foot = Inches(12);

  println!("One foot equals {:?}", foot);

  let meter = Centimeters(100.0);

  let cmp = if foot.to_centimeters() < meter {
    "smaller"
  } else {
    "bigger"
  };

  println!("One foot is {} than one meter.", cmp);
}

// Clone
// MEMO: コピーとクローンの違いを

//メモリ上の資源を扱う場合、変数束縛や関数呼び出しを介して移動させるのがデフォルトの挙動
// 場合によっては資源のコピーを作るのが適切なことがある

// Cloneトレイトはこのためにある
// 普通はCloneトレイトで定義されている .clone() を用いる

// いかなる資源も持たない構造体
#[derive(Debug, Clone, Copy)]
struct Nil;

// Clone トレイトを実装する型の変数を資源として持つタプル
#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
  // インスタンスを作成
  let nil = Nil;
  // nilをコピー、(移動させる資源は存在しない)
  let copied_nil = nil;

  // いずれのNilも独立に使用できる
  println!("original: {:?}", nil);
  println!("copy: {:?}", copied_nil);

  // Pairのインスタンスを作成
  let pair = Pair(Box::new(1), Box::new(2));
  println!("original {:?}", pair);

  // pairをコピー、資源は移動する
  let moved_pair = pair;
  println!("copy: {:?}", moved_pair);

  // 移動したので pair は所有権を失っている
  // println!("original {:?}", pair); // Error

  // moved_pairをcloned_pairにクローンする（資源もクローンされる）
  let cloned_pair = moved_pair.clone();

  // 使ってみる
  // println!("original {:?}", pair); // もともとの値なのでそりゃだめ
  println!("copy(moved): {:?}", moved_pair);
  println!("copy(cloned): {:?}", cloned_pair);

  // std::mem::dropを用いて元のpairをドロップする
  drop(moved_pair);
  // エラー! `moved_pair`はドロップされている。
  // println!("copy: {:?}", moved_pair);

  // .clone()した値はまだ使用可能！
  println!("clone: {:?}", cloned_pair);
}

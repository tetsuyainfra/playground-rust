// ジェネリック境界( Generic Bound )
// 型パラメータが特定の機能を持っていることを規定するためにトレイトに境界を設ける必要があることがよくある
// 以下の例ではDisplayトレイトを用いてprintlnするためTがDisplayを持っていることを規定している
// = TはDisplayを”実装していなくてはならない”

use std::fmt::Display;

// Display Traitを実装しているTを引数として取るジェネリック関数を定義
fn printer<T: Display>(t: T) {
  println!("{}", t);
}

// 境界はジェネリクスを全ての型ではなく一定条件を満たす型に対してのみ適用するためにある
// <T: Display>は<T>の部分集合であると意識すれば 「境界」の言葉の意味がしっくりくる

#[allow(dead_code)]
fn testing() {
  // DisplayTraitを境界としたジェネリック型Sを定義
  struct S<T: Display>(T);

  // Vec<T>
  // Vec<i32>は Displayを実装していないので
  // エラーになる
  // let s = S(vec![1]);
}

use std::fmt::Debug;

trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Rectangle {
  fn area(&self) -> f64 {
    self.length * self.height
  }
}

#[derive(Debug)]
struct Rectangle {
  length: f64,
  height: f64,
}

#[allow(dead_code)]
struct Triangle {
  length: f64,
  height: f64,
}

// ジェネリック型TはDebugトレイトを実装していなくてはならない
// その限りにおいてTがどのような型でも良い
fn print_debug<T: Debug>(t: &T) {
  println!("{:?}", t);
}

// TはHasAreaを実装しなくてはならない
// という境界条件を満たしていれば、HasAreaの関数area()にアクセスできる
fn area<T: HasArea>(t: &T) -> f64 {
  t.area()
}

fn main() {
  let rectangle = Rectangle {
    length: 3.0,
    height: 4.0,
  };
  let _triangle = Triangle {
    length: 3.0,
    height: 4.0,
  };

  print_debug(&rectangle);
  println!("Area: {}", area(&rectangle));

  // print_debug(&_triangle);
  // println!("Area: {}", area(&_triangle));
  // ^ TODO: これらの行をアンコメントしてみましょう。
  // | Error: `Debug` も `HasArea`もどちらも実装されていません!
}

// 下を定義すると area関数を使えるようになる
// impl HasArea for Triangle {
//   fn area(&self) -> f64 {
//     (self.length * self.height) / 2.0
//   }
// }

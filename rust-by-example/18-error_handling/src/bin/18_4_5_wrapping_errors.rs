// Wrapping errors
// An alternative to boxing errors is to wrap them in your own error type.
// ボックス化を使う代わりに、エラーをあなたのエラー型で包む

use std::error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
  EmptyVec,
  // エラーの解析エラーの実装に従います。
  // 追加情報を提供するには、タイプにデータを追加する必要があります。
  Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
      // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
      // これはラッパーなので fmt の基礎となる型の実装にしたがう
      DoubleError::Parse(ref e) => e.fmt(f),
    }
  }
}

impl error::Error for DoubleError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    //                                            ^^^^^^^
    match *self {
      DoubleError::EmptyVec => None,
      // The cause is the underlying implementation error type. Is implicitly
      // cast to the trait object `&error::Error`. This works because the
      // underlying type already implements the `Error` trait.
      // 原因は根本的な実装エラータイプです。 暗黙的に
      // トレイトオブジェクト `＆error :: Error`にキャストします。 これは、
      // 基本となる型はすでに `Error`トレイトを実装しています。
      DoubleError::Parse(ref e) => Some(e),
    }
  }
}

// From::from() の実装
// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
// `ParseIntError`から` DoubleError`への変換を実装します。
//   これは、 `ParseIntError`の場合、`？ `によって自動的に呼び出されます。
//   `DoubleError`に変換する必要があります。
impl From<ParseIntError> for DoubleError {
  fn from(err: ParseIntError) -> DoubleError {
    DoubleError::Parse(err)
  }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
  let first = vec.first().ok_or(DoubleError::EmptyVec)?; // <-- これは明示している
  let parsed = first.parse::<i32>()?; // <-- From::from()の実装で自動的に呼ばれる

  Ok(2 * parsed)
}

fn print(result: Result<i32>) {
  match result {
    Ok(n) => println!("The first doubled is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}

// これにより、エラーを処理するための定型が少し追加され、すべてのアプリケーションで必要になるとは限りません。
//  ボイラープレートを処理できるライブラリがいくつかあります。

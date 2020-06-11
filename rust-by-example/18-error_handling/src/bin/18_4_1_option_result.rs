// Pulling Results out of Options

use std::num::ParseIntError;

// 混合エラータイプを処理する最も基本的な方法
// それはお互いに埋め込むこと
//                                 __________________________________
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
  vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

// エラー（？など）で処理を停止したい場合がありますが、
// OptionがNoneの場合は続行します。 いくつかのコンビネータが結果とオプションを交換するのに便利です。
fn double_first_inv(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
  let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

  opt.map_or(Ok(None), |r| r.map(Some))
  // ↓と同じ意味
  // opt.map_or(Ok(None), |r| r.map(|v| Some(v)))
}

fn main() {
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {:?}", double_first(numbers));

  println!("The first doubled is {:?}", double_first(empty));
  // Error 1: the input vector is empty

  println!("The first doubled is {:?}", double_first(strings));
  // Error 2: the element doesn't parse to a number

  // Add
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];
  println!("The first doubled is {:?}", double_first_inv(numbers));
  println!("The first doubled is {:?}", double_first_inv(empty));
  println!("The first doubled is {:?}", double_first_inv(strings));
}

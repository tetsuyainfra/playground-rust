// ときどき、パニックの可能性なしにアンラップの単純さを求めます。
// これまで、unwrapは、変数を取得するために、より深くネストすることを余儀なくされていました。
//  これがまさに？の目的です。

// Errを見つけたら、2つの有効なアクションがあります。

// 1. panic! これは可能であれば回避する
// 2. return Errは処理できないことを意味する

// ?はErrsでパニックする代わりに戻るunwrapとほぼ同じです。
//  コンビネーターを使用した以前の例をどのように簡略化できるか見てみましょう。

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  let first_number = first_number_str.parse::<i32>()?; //  <---
  let second_number = second_number_str.parse::<i32>()?; // <---

  Ok(first_number * second_number)
}

// もうだめかも 1.43.1 2020/06/11
// 追加：tryバージョン
// fn multiply_try(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//   let first_number = try!(first_number_str.parse::<i32>()); //<--
//   let second_number = try!(second_number_str.parse::<i32>()); // <--

//   Ok(first_number * second_number)
// }

fn print(result: Result<i32, ParseIntError>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  print(multiply("10", "2"));
  print(multiply("t", "2"));
}

// try macro
// ? が存在する前は、同じ機能のtry! macroがありました。
// 現在は ? 演算子が推奨されていますが、まだ有効なので試してみてください！
// 古いコードを見るとき、 前の例と同じ乗算関数は、try！を使用すると次のようになります。

// alias for Result
// どのようにしてResult型を何度も使えばよいか
// ラストは別名の作成を我々に許している
// 便利なことに、我々は特別なResultを定義できる

// モジュールレベルでは、エイリアスを作成すると特に役立ちます。
//  特定のモジュールで検出されたエラーは同じErrタイプを持つことが多いため、
// 単一のエイリアスで関連するすべての結果を簡潔に定義できます。
// これは、stdライブラリがio::Result を提供するほど便利です。

// 構文を示す、かんたんな例を次に示す

use std::num::ParseIntError;

//  ParseIntErrorをErrの型に持つ全てのResultのジェネリックエイリアス
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
  first_number_str.parse::<i32>().and_then(|first_number| {
    second_number_str
      .parse::<i32>()
      .map(|second_number| first_number * second_number)
  })
}

// もう一度使用。エイリアスによって再度明記する必要性がない。
fn print(result: AliasedResult<i32>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  print(multiply("10", "2"));
  print(multiply("t", "2"));
}

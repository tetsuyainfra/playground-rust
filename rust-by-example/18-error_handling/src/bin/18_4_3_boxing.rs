// Box ing errors

// 元のエラーを保持しながら、シンプルなコードを書く方法はそれらをボックス化することです
// 欠点は根本的なエラー型が実行時にのみ認識され性的に決定されないこと

// stdlibは、エラーをボックス化するのに役立ちます。
// ~~~ Fromを介して、Errorトレイトを実装する任意の型から、
// トレイトオブジェクトBox <Error>への変換をBoxに実装させることで ~~~

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid first item to double")
  }
}

impl error::Error for EmptyVec {
  fn description(&self) -> &str {
    "invalid first item to double"
  }

  fn cause(&self) -> Option<&(dyn error::Error)> {
    None
  }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
  vec
    .first()
    .ok_or_else(|| EmptyVec.into())
    .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| i * 2))
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

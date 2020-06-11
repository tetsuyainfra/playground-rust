// Other uses of ?

// Notice in the previous example
//  that our immediate reaction
//  to calling parse is to map the error from a library error into a boxed error:
// 前の例で、
// parseの呼び出しに対する即時の反応は、
// ライブラリエラーからのエラーをボックス化されたエラーにマップすることであることに注意してください。

// .and_then(|s| s.parse::<i32>()
//     .map_err(|e| e.into())

// これは簡単で一般的な操作なので、省略できれば便利です。
// 悲しいかな、and_thenは十分に柔軟ではないため、柔軟ではありません。
// ただし、代わりに？を使用できます。

//  - ? -以前はunwrap()したものまたはErr(err)を返すと説明されていました。
//  これはほとんど真実です。 これは実際には、Err(From::from(err))をアンラップまたは返すことを意味します。
//  From::fromは異なるタイプ間の変換ユーティリティであるため、
// これは、 エラーが戻り値の型に変換可能な場合は、自動的に変換されます。
//                                           ~~~~~~~~~~~~~

// ここでは、前の例を？を使用して書き換えます。
// 結果として、エラータイプにFrom::fromが実装されている場合、
// map_errはなくなります。

use std::error;
use std::fmt;

// Change the alias to `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
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

  fn cause(&self) -> Option<&error::Error> {
    // Generic error, underlying cause isn't tracked.
    None
  }
}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
  let first = vec.first().ok_or(EmptyVec)?;
  let parsed = first.parse::<i32>()?;
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

// これは実際にはかなりきれいです。
// 元のパニックと比較すると、unwrap呼び出しを？に置き換えるのとよく似ています。
// ただし、戻り値の型はResultです。 その結果、それらは最上位レベルで分解されなければなりません。

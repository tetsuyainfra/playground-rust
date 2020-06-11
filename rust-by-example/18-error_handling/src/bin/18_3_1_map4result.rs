// map for Result

// 前回の例のpanicではmultiplyは頑強なコードにならない
// 一般的に我々はエラーを返して、エラーに応答する正しい方法を決定できるようにする。

// まず、どの種類のエラーを処理しているかを知る必要があります。
// Errタイプを判別するには、i32のFromStr Traitで実装されているparse()を調べます。
// その結果、ErrタイプはParseIntErrorとしてしていされています。

// 以下の例では、単純明快なmatchステートメントにより、全体的に扱いにくいコードになります。
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  match first_number_str.parse::<i32>() {
    Ok(first_number) => match second_number_str.parse::<i32>() {
      Ok(second_number) => return Ok(first_number * second_number),
      Err(e) => return Err(e),
    },
    Err(e) => return Err(e),
  }
}
// and_then, map を活用した場合
fn multiplyMap(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  first_number_str.parse::<i32>().and_then(|first_number| {
    //                            ~~~~~~~~
    second_number_str
      .parse::<i32>()
      .map(|second_number| first_number * second_number)
    // ~~~
  })
}

fn print(result: Result<i32, ParseIntError>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: is {}", e),
  }
}

fn main() {
  let twenty = multiply("10", "2");
  print(twenty);

  let tt = multiply("t", "2");
  print(tt);

  let fourty = multiplyMap("20", "2");
  print(fourty)
}

// 幸い、Optionのマップ、and_then、およびその他の多くのコンビネーターもResultに実装されています。
// 結果には完全なリストが含まれています。

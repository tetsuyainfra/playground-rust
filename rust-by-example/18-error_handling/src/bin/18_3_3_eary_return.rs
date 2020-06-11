// Early return 早期リターン
// 前の例では、コンビネーターを使用して明示的にエラーを処理しました。
// このケース分析を処理する別の方法は、一致ステートメントと早期リターンを組み合わせて使用することです。

// つまり、関数の実行を停止して、エラーが発生した場合にエラーを返すだけです。
//  一部の人にとって、この形式のコードは、読み取りと書き込みの両方がより簡単になる場合があります。
// 前の例のこのバージョンを考えてください。早期のリターンを使用して書き直されます。
use std::num::ParseIntError;

// 早期リターンバージョン
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  let first_number = match first_number_str.parse::<i32>() {
    Ok(first_number) => first_number,
    Err(e) => return Err(e),
  };

  let second_number = match second_number_str.parse::<i32>() {
    Ok(second_number) => second_number,
    Err(e) => return Err(e),
  };

  Ok(first_number * second_number)
}

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

// この時点で、コンビネーターと早期リターンを使用してエラーを明示的に処理する方法を学びました。 通常、パニックを回避したいのですが、すべてのエラーを明示的に処理するのは面倒です。

// 次のセクションでは、？ おそらくパニックを引き起こさずにアンラップする必要がある場合のために。

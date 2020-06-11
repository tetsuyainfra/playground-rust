// Result は Option型のよりリッチなバージョンで、存在しない可能性の代わりにエラーの記述を可能にする

// Result<T,E>の二つの結果になる
// - Ok<T> : 要素Tが見つかった場合
// - Err<E> : 要素Eとともにエラーが見つかった場合

// 慣例で予期される結果はOk<T>に対し、予期される結果はErrである

// Optionと同様にResultは多くのメソッドが関連付けられている
// unwrap() は 要素Tあるいはpanicを生成する
// caseのとりあつかいはResultとOptionに共通のコンビネータがある

// fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
//   // Let's try using `unwrap()` to get the number out. Will it bite us?
//   let first_number = first_number_str.parse::<i32>().unwrap();
//   let second_number = second_number_str.parse::<i32>().unwrap();
//   first_number * second_number
// }

// fn main() {
//   let twenty = multiply("10", "2");
//   println!("double is {}", twenty);

//   let tt = multiply("t", "2");
//   println!("double is {}", tt);
// }

// 失敗した場合にはparse()のエラーでunwrap()してpanicを起こし、プログラムを終了し、エラーメッセージを残す

// エラーメッセージの品質を向上させるには
// 戻り値の型をより具体的にし、エラーを明示的に処理することを検討する必要がある

// main 内でのResultの使用
// もしmainがResult型を返すこともできる
// もしmain内でエラーが起きたら、エラーコードを返して、そのエラーのデバッグ表示を出力する。(Debug traitを利用する)
// 次の例は、このようなシナリオを示し、次のセクションで説明する側面に触れています。

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
  let number_str = "10";
  let number = match number_str.parse::<i32>() {
    Ok(number) => number,
    Err(e) => return Err(e),
  };

  println!("{}", number);

  Ok(())
}

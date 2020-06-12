// プログラムの一部が失敗した際、panic!するよりも、エラーを補足する方が望ましい場合があります。
// これはOptionという列挙型を用いることで可能になります。

// 列挙型Option<T>には2つの値があります。
// - None、これは実行の失敗か値の欠如を示します。
// - Some(value)、型Tのvalueをラップするタプルです。

// 失敗を起こさない整数の割り算
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
  if divisor == 0 {
    None
  } else {
    Some(dividend / divisor)
  }
}

// 失敗する割り算を扱うことができる
fn try_division(dividend: i32, divisor: i32) {
  match checked_division(dividend, divisor) {
    None => println!("{} / {} failed!", dividend, divisor),
    Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
  }
}

fn main() {
  try_division(4, 2);
  try_division(1, 0);

  // `None`を変数にアサインする際は、型を明示しなくてはならない。
  let none: Option<i32> = None;
  let _equivalent_none = None::<i32>;

  let optional_float = Some(0f32);

  // `Some`をアンラップすると中の値を取得できる。
  println!(
    "{:?} unwraps to {:?}",
    optional_float,
    optional_float.unwrap()
  );

  // Unwrapping a `None` variant will `panic!`
  // `None`をアンラップしようとすると`panic!`る
  println!("{:?} unwraps to {:?}", none, none.unwrap());
}

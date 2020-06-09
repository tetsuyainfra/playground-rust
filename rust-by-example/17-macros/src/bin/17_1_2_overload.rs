// マクロは異なる引数の組み合わせを取るようにオーバーロードすることができる
// macro_rules!はマッチと似たような使い方をすることができます。

// test!は$left, $rightを異なる呼び出し方に応じて比較する

macro_rules! test {
  ($left: expr; and $right : expr) => {
    println!(
      "{:?} and {:?} is {:?}",
      stringify!($left),
      stringify!($right),
      $left && $right
    )
  };
  ($left: expr; or $right: expr) => {
    println!(
      "{:?} or {:?} is {:?}",
      stringify!($left),
      stringify!($right),
      $left || $right
    )
  };
}

fn main() {
  test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
  test!(true; or false);
}

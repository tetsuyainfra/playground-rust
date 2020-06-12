// リピート（Repeatation）

// マクロは引数のリストの中で+を使うことができ、
// そうすることによって、引数が少なくとも1回以上繰り返されるということを示すことができます。
// 同様に*の場合は、0以上を示します。

// 以下の例では、 マッチ対象を $(...),+で囲むことにより、 カンマで区切られた1つ以上の式とマッチします。
// 最後のセミコロンは必須ではない

macro_rules! find_min {
  ($x: expr) => {
    $x
  };

  // $x に少なくとも1つの$yが続く場合
  ($x: expr, $($y: expr), +) => (
    //       ^^^^^^^^^^^^^^
    std::cmp::min($x, find_min!($($y),+))
  )
}

fn main() {
  println!("{}", find_min!(1u32));
  println!("{}", find_min!(1u32 + 2, 2u32));
  println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
// `allow` required to silence warnings because only
// one variant is used.
// `allow`は値を一つだけ使用したことによる警告を抑えるために存在する。
#[allow(dead_code)]

enum Color {
  // These 3 are specified solely by their name.
  // これら3つの値は名前のみで扱うことができる
  Red,
  Blue,
  Green,
  // These likewise tie `u32` tuples to different names: color models.
  // 以下の値は名前と`u32`のタプルをペアにしている。
  // カラーモデルと呼ばれる。
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}

fn main() {
  let color = Color::RGB(122, 17, 40);
  // TODO ^ Try different variants for `color`
  // TODO ^ `Color`に別の変数を入れてみましょう

  println!("What color is it?");
  // An `enum` can be destructured using a `match`.
  // `enum`は`match`を利用してデストラクトすることができる。
  match color {
    Color::Red => println!("The color is Red!"),
    Color::Blue => println!("The color is Blue!"),
    Color::Green => println!("The color is Green!"),
    Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
    Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
    Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
    Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
    Color::CMYK(c, m, y, k) => println!(
      "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
      c, m, y, k
    ),
    // Don't need another arm because all variants have been examined
    // 全ての値を列挙したのでその他の場合の処理は必要ない。
  }
}

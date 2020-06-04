// Statement

// Rustはほとんどの場合、文(Statement)の連続で出来ている
// fn main() {
//   // statement
//   // statement
//   // statement
// }

// 宣言文にはいくつか種類がある  一般的なのは
// - variable binding
//   - 変数の束縛
// - expression
//   - ; 付きの式
//
// fn main() {
//   // variable binding
//   // 変数束縛
//   let x = 5;
//
//   // expression;
//   // 式;
//   x;
//   x + 1;
//   15;
// }

// コードブロックも式の一種
// そのため、ブロックを丸ごと値として扱える
// - ブロックの最後の式が場所を表す式に代入される <-- この表現よくわからん・・・意味はわかるけど
// - ;で最後を終えると () が変える値となる
//
fn main() {
  let x = 5u32;

  // 式
  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // This expression will be assigned to `y`
    // この式は`y`に代入されます。
    x_cube + x_squared + x
  };

  #[allow(unused_must_use)]
  let z = {
    // The semicolon suppresses this expression and `()` is assigned to `z`
    // セミコロンがあるので`z`には`()`が入ります。
    // #[allow(unused_must_use)]
    2 * x;
  };

  println!("x is {:?}", x); // 5
  println!("y is {:?}", y); // 155
  println!("z is {:?}", z); // ()
}

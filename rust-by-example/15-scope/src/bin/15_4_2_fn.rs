// 関数

// ライフタイムパラメータを"省略をしない"場合、
// ライフタイムのシグネチャ(e.g. <'a>)を持つ関数にはいくつかの制限がある
// - 全ての変数においてライフタイムを明示しなくてはならない。
// - 返り値となる参照はすべて引数と同じライフタイムか、staticライフタイムを持たなくてはならない

// 引数として`'a`のライフタイムで参照を一つ取る。最低でもこの関数
// と同じだけの長さでなくてはならない。
fn print_one<'a>(x: &'a i32) {
  println!("`print_one`: x is {}", x);
}

// ミュータブルな参照でも同様
fn add_one<'a>(x: &'a mut i32) {
  *x += 1;
}

// 異なるライフタイムを持つ複数の引数がある場合。
// ここでは1種類のライフタイムでも問題はないが、より複雑なケースでは
// 異なるライフタイムが必要になる場合がある。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("`print_multi`: x is {}, y is {}", x, y);
}

// 受け取った参照をそのまま返すことに問題はないが、適切なライフタイム
// でなくてはならない。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
  x
}

// これは引数と同じライフタイムでない（or'static）じゃないのでエラーになる
// fn pass_xb<'a, 'b>(x: &'a i32, _: &'a i32) -> &'b i32 {
//   x
// }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// `'a`は関数より長くなくてはならないため上の関数は正しくない。
// ここでは、`&7`は`i32`のデータとそれへの参照を作り出す。
// その後データはスコープを抜けるとともに破棄される。そのため、
// 不適切なデータに対する参照を返すことになってしまう。

fn main() {
  let x = 7;
  let y = 9;

  print_one(&x);
  print_multi(&x, &y);

  let z = pass_x(&x, &y);
  print_one(z);

  let mut t = 3;
  add_one(&mut t);
  print_one(&t);
}

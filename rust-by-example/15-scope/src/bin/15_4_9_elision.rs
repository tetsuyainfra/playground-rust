// 省略 elision

// ライフタイムのパターンのいくつかは一般的に使用されるため
// タイプ量を減らし可読性を上げるために省くことができる
// これを省略という。
// パターンが一般的だという理由で存在する

//elided_input のライフタイムはコンパイラによって自動付与される
// 以下の2つは同一のライフタイムシグネチャ
fn elided_input(x: &i32) {
  println!("`elided_input`: {}", x);
}
fn annotated_input<'a>(x: &'a i32) {
  println!("`annotated_input`: {}", x);
}

// 同様に以下の2つの関数も同じライフタイムシグネチャを持つ
fn elided_pass(x: &i32) -> &i32 {
  x
}
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
  x
}

fn main() {
  let x = 3;

  elided_input(&x);
  annotated_input(&x);

  println!("`elided_pass`: {}", elided_pass(&x));
  println!("`annotated_pass`: {}", annotated_pass(&x));
}

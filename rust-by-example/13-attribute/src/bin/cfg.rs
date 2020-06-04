// cfg
// 環境に応じたコンパイルをする方法は2種類ある
// - cfg attirbute: #[cfg(...)]をアトリビュートとして使用する
// - cfg! macro   : cfg!(...)をブーリアンとして評価する
// いずれも 適切なシンタックスで記述する必要がある

// これはアトリビュート
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
  println!("You are running linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
  println!("You are running linux");
}

fn main() {
  are_you_on_linux();

  println!("Are you sure?");
  // これはcfg!マクロを利用している
  if cfg!(target_os = "linux") {
    println!("Yes. It's definitely linux!");
  } else {
    println!("Yes. It's definitely *not* linux!");
  }
}

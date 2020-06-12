// スレッド
// Rustはspawn関数を用いてOSのネイティブスレッドを開始することができます。
// この関数の引数はmoveクロージャ
// （訳注: 参照ではなく値を取るクロージャ。　詳しくは[クロージャを返す関数][fn_output]を参照）です。

use std::thread;

static N_THREADS: i32 = 10;

fn main() {
  let mut children = vec![];

  // 0 <= i < 10
  for i in 0..N_THREADS {
    children.push(thread::spawn(move || {
      println!("this is thread number {}", i)
    }))
  }

  for child in children {
    let _ = child.join();
  }
}

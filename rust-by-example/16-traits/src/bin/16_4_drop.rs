// メモリ解放

// /%aaropトレイトにはメソッドがdropメソッド一つだけしかにあ。
// これはオブジェクトがスコープから抜けた時に自動で呼ばれる
// Dropトレイトの主な使用目的はインスタンスが所有する資源を解放すること

// Dropトレイとを実装している型の例として
// Box, Vec, String, File, Processがあげられる

// Dropトレイとは任意の型に手動で実装できる

// 以下の例ではdropメソッドにコンソール出力を追加することでメソッドが呼ばれたタイミングがわかる

struct Droppable {
  name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
// このちょっとした実装で、`drop`にコンソール出力機能がつきます。
impl Drop for Droppable {
  fn drop(&mut self) {
    println!("> Dropping {}", self.name);
  }
}

fn main() {
  let _a = Droppable { name: "a" };

  // block A
  {
    let _b = Droppable { name: "b" };

    // block B
    {
      let _c = Droppable { name: "c" };
      let _d = Droppable { name: "d" };

      println!("Exiting block B");
    }
    println!("Just exited block B");

    println!("Exiting block A");
  }
  println!("Just exited block A");

  // Variable can be manually dropped using the `drop` function
  // `drop`関数を用いて変数を手動で開放することもできます。
  drop(_a);
  // TODO ^ Try commenting this line
  // TODO ^ この行をコメントアウトしてみましょう。

  println!("end of the main function");

  // `_a` *won't* be `drop`ed again here, because it already has been
  // (manually) `drop`ed
  // `_a`はここで`drop`されることは *ない* 。なぜならば、上ですでに
  // （手動で）`drop`されているため。
}

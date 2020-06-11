// panic
// 最もシンプルなエラーハンドリングメカニズム
// これはエラーメッセージを出力し、スタックの巻き戻しを開始し、通常はプログラムを終了します。
// ここではpanic、エラー条件を明示的に呼び出します。

fn give_princess(gift: &str) {
  if gift == "snake" {
    panic!("AAAaaaaaa!!!!");
  }
  println!("I love {}s!!!!!", gift);
}

fn main() {
  give_princess("teddy bear");
  give_princess("snake");
}

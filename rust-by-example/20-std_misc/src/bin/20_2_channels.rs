// チャネル
// Rustは、スレッド間のコミュニケーションのために、非同期のチャネル(channels)を提供しています。
// チャネル2つのエンドポイント、すなわち送信者(Sender)と受信者(Receiver)を介して、
// 情報の一方向への流れを作り出すことを可能にしています。

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static N_THREADS: i32 = 3_i32;

fn main() {
  // チャンネルにはSender<T>とReciever<T>という２つのエンドポイントがある
  // Tは送信されるメッセージの型である
  // 型アノテーションは必須ではない
  let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

  let mut children = Vec::new();

  for id in 0..N_THREADS {
    // 送信者のエンドポイントはこぴーすることができる
    let thread_tx = tx.clone();

    let child = std::thread::spawn(move || {
      // スレッドはthread_txの所有権をとり、
      // それぞれのスレッドはメッセージをチャンネルにキューイングする
      thread_tx.send(id).unwrap();

      println!("thread {} finished", id);
    });

    children.push(child);
  }

  // ここですべてのメッセージのが収集される
  let mut ids = Vec::with_capacity(N_THREADS as usize);
  for _ in 0..N_THREADS {
    // recvメソッドはチャンネルからメッセージを取り出す
    // もし取り出せるメッセージが存在しない場合、recvは現在のスレッドをブロックする
    ids.push(rx.recv())
  }

  // すべてのスレッドの終了を待つ
  for child in children {
    child.join().expect("oops! the child thread panicked");
  }

  println!("{:?}", ids);
}

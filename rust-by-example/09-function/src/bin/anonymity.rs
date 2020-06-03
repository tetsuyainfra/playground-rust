// FはGeneric型でなくてはならない
fn applyFnOnce<F>(f: F)
where
  F: FnOnce(), // FはFnOnceを実装していなくてはならない
               // Fnは引数と返りを持たない
{
  f();
}

// クロージャが定義されるとコンパイラは裏側で無名の構造体を作る
// そこにクロージャで使われる外側の変数を入れる
// 同時にFn, FnMut, FnOnceという名のトレイトのいずれか一つを介して構造体に関数としての機能を実装する
// そして実際に呼ばれるまで待つ
// この無名構造体は型が未指定(unknown)なため、関数を実行するにはジェネリクスが必要
// とはいえ<T>だけでは曖昧 &self, &mut self, selfのいずれを引数に取るかわからないので
// そのためFn, FnMut, FnOnceのいずれか一つを実装することで対応している

// FはGeneric型でなくてはならない
fn apply<F>(f: F)
where
  F: Fn(), // FはFnを実装していなくてはならない
           // Fnは引数と返りを持たない
{
  f();
}

fn main() {
  let x = 7;

  // x が無名の構造体に入れられそれに対してFnを実装する
  // Fn は fn Fn(&self) -> { println!("{}", &self); }
  // selfは無名構造体の実体
  // その構造体がprintに入る
  let print = || println!("{}", x);

  apply(print);
}

// クロージャでない普通の関数を引数として渡すことは可能か
// -> 可能
//    パラメータとしてクロージャを取る関数を定義すれば
//    そのクロージャのトレイと境界を満たす任意の関数をパラメータとして受け取れる

// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
// 関数を引数として取り、即座に実行する関数を定義
fn call_me<F: Fn()>(f: F) {
  f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
  println!("I'm a function!");
}

fn main() {
  // Define a closure satisfying the `Fn` bound
  let closure = || println!("I'm a closure!");

  call_me(closure);
  call_me(function);
}

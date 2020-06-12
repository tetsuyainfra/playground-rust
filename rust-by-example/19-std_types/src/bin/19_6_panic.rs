// panic!マクロはパニックを生成し、スタックの巻き戻しを開始するために使用することができます。
// 巻き戻しの間、ランタイムは、（訳注: panicを起こした）スレッドが 所有権を持つ
//  全ての資源のデストラクタを呼び出し、メモリ上から解放します。

// 今回はシングルスレッドのプログラムを実行しているので、panic!はプログラムにパニックメッセージを表示させ、exitします。
// Re-implementation of integer division (/)
// 整数の除法(/)の再実装
fn division(dividend: i32, divisor: i32) -> i32 {
  if divisor == 0 {
    // Division by zero triggers a panic
    // ゼロによる除算はパニックを引き起こす
    panic!("division by zero");
  } else {
    dividend / divisor
  }
}

// The `main` task
// `main`のタスク
fn main() {
  // Heap allocated integer
  // ヒープ上の整数
  let _x = Box::new(0i32);

  // This operation will trigger a task failure
  // このオペレーションはタスクの失敗を引き起こす
  division(3, 0);

  println!("This point won't be reached!");

  // `_x` should get destroyed at this point
  // `_x`はここに到達する前に破棄される。
}

// Borrowing 借用
// データの所有権を完全に受け渡すことなく一時的にアクセスしたい場合がほとんど
// Rustでは借用(borrowing)で実現する
// 値そのもの(val: T)を受け渡すのではなく(val: &T)を渡す

// コンパイラは借用チェッカを用いてリファレンスが
// 常に有効なオブジェクトへの参照であることを
// "コンパイル時に保証する"
// オブジェクトのリファレンスが存在 ＝ オブジェクトは破壊できない

// この関数はボックスの所有権を奪い破壊する
fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("This int is {}", borrowed_i32);
}

fn main() {
  let boxed_i32 = Box::new(5i32);
  let stacked_i32 = 6i32;

  // Boxの中身を借用。所有権を奪うわけではないため、
  // 直後にもう一度借用できる。
  borrow_i32(&boxed_i32);
  borrow_i32(&stacked_i32);

  {
    // ボックス内の要素に対する参照を取得
    let _ref_to_i32: &i32 = &boxed_i32;

    // ボックス内の要素が借用されているため、`boxed_int`を破棄する
    // ことはできない。
    // eat_box_i32(boxed_i32); // Error
    // FIXME ^ この行をコメントアウトしましょう。

    borrow_i32(_ref_to_i32);
    // ここで`_ref_to_int`はスコープを抜け、借用もなくなります。
  }

  // ここでようやく、`eat_box`は所有権を移譲し、破棄することができます。
  eat_box_i32(boxed_i32);
}

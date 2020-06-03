fn main() {
  use std::mem;

  // ----
  let color = "green";

  // `color`をプリントするためのクロージャ。
  // これは`color`を借用(`&`)し、その借用とクロージャを`print`
  // という名の変数に保持する。
  // 借用は`print`がスコープから出るまで続く。
  // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
  let print = || println!("`color`: {}", color);

  print();

  // colorは不変で借用できる
  // printクロージャーは参照を保持しているだけだからだ
  let _reborrow = &color;
  print();

  // 移動してもよい
  let _color_moved = color;
  print();

  // ----
  let mut count = 0;
  // `count`をインクリメントするためのクロージャ。`count`と`&mut count`
  // の両方を取ることができるが、後者のほうが制限が少ないため、
  // （訳注: `count`だと`&mut count`と違い、一度しか呼ぶことができない。）
  // そちらを取る。直後に`count`を借用する。
  //
  // `inc`には`mut`をつける必要がある。なぜならミュータブルな型が
  // 中で使用されているからである。ミュータブルなクロージャは呼ぶたびに
  // 内部変数を変更する
  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  };

  inc();
  //
  // のでエラーが起きる
  // let _reborrow = &count;
  inc();
  //
  let _count_reborrowed = &mut count;

  let movable = Box::new(3);

  // `mem::drop`は`T`（ジェネリック型）を取る必要があるため、このクロージャは
  // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
  // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。
  let consume = || {
    println!("`moved`: {:?}", movable);
    mem::drop(movable);
  };

  // `consume`は変数を消費（開放）するため、一度しか呼び出すことができない。
  consume();
  // consume(); -> error

  // `Vec` has non-copy semantics.
  let haystack = vec![1, 2, 3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // println!("There're {} elements in vec", haystack.len());
  // ^ Uncommenting above line will result in compile-time error
  // because borrow checker doesn't allow re-using variable after it
  // has been moved.
  //クロージャ内に移動してるので再利用できない

  // Removing `move` from closure's signature will cause closure
  // to borrow _haystack_ variable immutably, hence _haystack_ is still
  // available and uncommenting above line will not cause an error.
}

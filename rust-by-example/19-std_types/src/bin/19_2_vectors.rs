// ベクタ型
// 「ベクタ」はサイズを変更可能な配列です。スライスと同様、そのサイズはコンパイル時には不定ですが、いつでも要素を追加したり削除したりすることができます。ベクタは3つの要素で、その特徴が完全に決まります。
// - データへのポインタ
// - 長さ
// - 容量  あらかじめメモリ上にベクタのために確保された領域
// ベクタはその容量を超えない限りにおいて長くしていくことができます。超えた場合には、より大きな容量を持つように割り当てなおされます。

fn main() {
  // Iterators can be collected into vectors
  // イテレータは要素を収集してベクタにすることができる。
  let collected_iterator: Vec<i32> = (0..10).collect();
  // let mut collected_iterator: Vec<i32> = (0..10).collect(); // mutableなベクタ 書き換え可能
  println!("Collected (0..10) into: {:?}", collected_iterator);

  // The `vec!` macro can be used to initialize a vector
  // ベクタの初期化には`vec!`マクロが使用できる。
  let mut xs = vec![1i32, 2, 3];
  println!("Initial vector: {:?}", xs);

  // Insert new element at the end of the vector
  // 新しい要素をベクタの最後に挿入することができる。
  println!("Push 4 into the vector");
  xs.push(4);
  println!("Vector: {:?}", xs);

  // Error! Immutable vectors can't grow
  // エラー！イミュータブルなベクタは成長できない
  // collected_iterator.push(0);
  // FIXME ^ Comment out this line
  // FIXME ^ この行をコメントアウトしましょう。

  // The `len` method yields the number of elements currently stored in a vector
  // `len`メソッドは現在のベクタのサイズを返す。
  println!("Vector length: {}", xs.len());

  // Indexing is done using the square brackets (indexing starts at 0)
  // 鍵括弧(`[]`)を用いてインデックスによる要素へのアクセスができる
  // （インデックスは0から開始する）
  println!("Second element: {}", xs[1]);

  // `pop` removes the last element from the vector and returns it
  // `pop`はベクタの最後の要素を削除すると同時に返す。
  println!("Pop last element: {:?}", xs.pop());

  // Out of bounds indexing yields a panic
  // 不正なインデックスアクセスはpanicを引き起こします。
  println!("Fourth element: {}", xs[3]);
  // FIXME ^ Comment out this line

  // `Vector`s can be easily iterated over
  println!("Contents of xs:");
  for x in xs.iter() {
    println!("> {}", x);
  }

  // A `Vector` can also be iterated over while the iteration
  // count is enumerated in a separate variable (`i`)
  for (i, x) in xs.iter().enumerate() {
    println!("In position {} we have value {}", i, x);
  }

  // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
  // over in a way that allows modifying each value
  for x in xs.iter_mut() {
    *x *= 3;
  }
  println!("Updated vector: {:?}", xs);
}

use std::mem;

// This function borrows a slice
// この関数はスライスを借用する
fn analyze_slice(slice: &[i32]) {
  println!("first element of the slice: {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

fn main() {
  // Fixed-size array (type signature is superfluous)
  // 固定長の配列（型シグネチャは冗長なので、なくても可）
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  let xs = [1, 2, 3, 4, 5];

  // All elements can be initialized to the same value
  // すべての要素を0にする場合
  let ys: [i32; 500] = [0; 500];

  // Indexing starts at 0
  // インデックスは０から
  println!("first element of the array: {}", xs[0]);
  println!("second element of the array: {}", xs[1]);

  // `len` returns the size of the array
  // `len`は配列のサイズを返す。
  println!("array size: {}", xs.len());

  // Arrays are stack allocated
  // 配列はスタック上に置かれる
  println!("array occupies {} bytes", mem::size_of_val(&xs));

  // Arrays can be automatically borrowed as slices
  // 配列は自動的にスライスとして借用される。
  println!("borrow the whole array as a slice");
  analyze_slice(&xs);

  // Slices can point to a section of an array
  // They are of the form [starting_index..ending_index]
  // starting_index is the first position in the slice
  // ending_index is one more than the last position in the slice
  // スライスは配列の一部セクションを指すことが可能
  // 始まりと終わりで指定できる
  // 終わりのインデックスは最後からもう一つ次のポジションを指定する
  println!("borrow a section of the array as a slice");
  analyze_slice(&ys[1..4]); // 2,3,4,
  println!("{:?}", &ys[1..4]);
  analyze_slice(&ys[1..2]); // 2
  println!("{:?}", &ys[1..2]);

  // Out of bound indexing causes compile error
  // インデックスの範囲が配列のサイズを超えた場合パニックする
  // println!("{}", xs[5]);
}

// エイリアス

// ここのページはライン毎にコンパイルを試していく方がよいかも
// ていうかblockつかって制御しないと分かりにくくなるのでは？うーん？

// データは一度にいくつでもイミュータブルに借用できるが、
// その間、オリジナルのデータはミュータブルに借用できない
// -> https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/mutability.html
// > 次の2種類の借用のどちらか1つを持つことはありますが、両方を同時に持つことはありません。
// > リソースに対する1つ以上の参照（&T）
// > ただ1つのミュータブルな参照（&mut T）
// 複数のイミュータブル or 単一のミュータブルってやつ

// オリジナルのデータをもう一度借用できるのはミュータブルな参照が最後に使われた場所より後でなければならない

struct Point {
  x: i32,
  y: i32,
  z: i32,
}

fn main() {
  let mut point = Point { x: 0, y: 0, z: 0 };
  let borrowed_point = &point;
  let another_borrow = &point;

  // データは元々の持ち主と参照の両方からアクセスできる
  println!(
    "Point has coordinates: ({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z
  );

  // すでにイミュータブルに参照されているので
  // ミュータブルに参照できない
  // ※すでに参照されているのでミュータブルに参照できないでも良いのでは？
  // let mutable_borrow = &mut point;

  println!(
    "Point has coordinates: ({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z
  );
  // !!!!
  // ここより下でborrowd_point, another_borrowを使わない
  // するとイミュータブルな参照はないことになる
  // !!!!

  let mutable_borrow = &mut point;

  // ミュータブルな参照を介して値を変更する
  mutable_borrow.x = 5;
  mutable_borrow.y = 5;
  mutable_borrow.z = 5;

  // let y = &point.y;
  //  ココより下にイミュータブルな参照が無いときに↑を有効にしてもよい
  // それはイミュータブルな参照とミュータブルな参照が前後することになる？

  // イミュータブルな参照を取るのでprintできない
  // println!("Point Z coordinate is {}", point.z);
  //  ココより下が無いときに↑を有効にしてもよい
  // それはイミュータブルな参照とミュータブルな参照が前後することになる？

  println!(
    "Point has coordinates: ({}, {}, {})",
    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
  );

  // The mutable reference is no longer used for the rest of the code so it
  // is possible to reborrow
  let new_borrowed_point = &point;
  println!(
    "Point now has coordinates: ({}, {}, {})",
    new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
  );
}

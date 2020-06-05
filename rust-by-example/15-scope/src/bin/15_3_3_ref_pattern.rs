// refパターン

// let を介してデストラクタやパターンマッチングする場合、
// ref キーワードを用いて構造体、タプルフィールドへのリファレンスを取得できる

// 有用な例

#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let c = 'Q';

  // どちらも等価
  let ref ref_c1 = c;
  let ref_c2 = &c;

  println!("lef_c1 equals lef_c2: {}", *ref_c1 == *ref_c2);

  let point = Point { x: 0, y: 0 };

  let _copy_of_x = {
    // デストラクチャリングしてて xとyの値を ref_to_xと_に割り当ててる
    // そして、point.x への参照として ref_to_xに束縛している
    let Point {
      x: ref ref_to_x,
      y: _,
    } = point;

    // その参照をコピーして返している(コピーするかどうかは型による)
    // 基本型はコピーのはず
    *ref_to_x
  };

  // 書き換え可能な変数として束縛
  let mut mutable_point = point; //　Copyトレイトを定義しているのでMoveでなくコピー
  {
    let Point {
      x: _,
      y: ref mut mut_ref_to_y, // mutable_point.yのミュータブルな参照をmut_ref_to_yに束縛
    } = mutable_point;

    // 書き換え可能な参照を用いて書き換える
    *mut_ref_to_y = 1
  }

  println!("point is ({}, {})", point.x, point.y);
  println!("mutable_point is({}, {})", mutable_point.x, mutable_point.y);

  let mut mutable_tuple = (Box::new(5u32), 3u32);
  {
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;
  }
  println!("mutable_tuple is {:?}", mutable_tuple);
}

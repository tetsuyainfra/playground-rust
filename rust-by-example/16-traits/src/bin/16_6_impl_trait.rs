// impl Trait
// 関数で自作のトレイト(例えばMyTrait)を埋め込んだ型を返す時
// 返す方に -> impl MyTrait を指定できる
// これは型シグネチャをシンプルに書くことを助けてくれる

use std::iter;
use std::vec::IntoIter;

// この関数は2つのVec<i32>を結合してイテレータを返すが
// とても複雑な型になる
fn combine_vecs_explicit_return_type(
  v: Vec<i32>,
  u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
  v.into_iter().chain(u.into_iter()).cycle()
}

// これは同じ関数だがimpl Traitを利用してとてもシンプルな型になる
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
  v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
  let v1 = vec![1, 2, 3];
  let v2 = vec![4, 5];

  let mut v3 = combine_vecs(v1, v2);

  assert_eq!(Some(1), v3.next());
  assert_eq!(Some(2), v3.next());
  assert_eq!(Some(3), v3.next());
  assert_eq!(Some(4), v3.next());
  assert_eq!(Some(5), v3.next());
  println!("all done");

  // 追加
  let plus_one = make_adder_function(1);
  assert_eq!(plus_one(2), 3);
  println!("all done 2");

  // 追加2
  let v4 = vec![1, 2, 3];
  let mut v4 = double_positives(&v4);
  assert_eq!(Some(2), v4.next());
  assert_eq!(Some(4), v4.next());
  assert_eq!(Some(6), v4.next());
  println!("all done 3");
}

// さらに重要なこととして、Rustの一部の型は書き出せない
// たとえばすべてのクロージャーには独自の名前のない具象型がある。
// impl Trait構文の前にクロージャーを返すためにヒープに割り当てる必要があります。
// 今は次のようにすべてを静的に行える
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
  let closure = move |x: i32| x + y;

  closure
}

// クロージャimpl Traitを使用するイテレータを返すために使用することもできます！
// これは、使用して作ると簡単に。
// クロージャー型には名前がないため、関数がクロージャーを持つイテレーターを返す場合、
// 明示的な戻り型を書き出すことはできません。
// しかし、これを簡単に行うことができます

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
  numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

// たぶん・・・
// 参照でIteratorを使っているので型ライフタイムが必要

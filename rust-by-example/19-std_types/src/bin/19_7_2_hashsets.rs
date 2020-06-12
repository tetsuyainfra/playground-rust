// ハッシュ集合
// 値がなく、キーだけのHashMapを想像してみてください。
// これはハッシュ集合(HashSet)と呼ばれるものです。（HashSet<T>は、実際にはHashMap<T, ()>のラッパーです。）

// 「何の意味があるの？フツーにキーをVecに入れればいいじゃん」そう思いましたね？

// それは、HashSet独自の機能として、要素に重複がないということが保証されるためです。
// これは全ての集合(set)型がもつ機能です。HashSetはその実装の1つであり、他にはBTreeSet等があります。

// HashSetに、すでに存在する値を加えようとすると、
// （すなわち、加えようとしている値のハッシュ値と、要素中のいずれかの値のハッシュ値が等しい場合、）
// 新しい値によって古い値が上書きされます。

// これは、同じ値を2つ以上欲しくない場合や、すでにある値を持っているか知りたい場合にとても有効です。

// しかし、集合型の機能はそれだけではありません。

// 集合型には4つの主要なメソッドがあり、（すべてイテレータを返します。）
// - union: 2つの集合型のどちらか一方にある値を全て取得
// - difference: 1つ目の集合にあり、かつ2つ目には存在しない値を全て取得。
// - intersection: 両方の集合にある値のみを取得。
// - symmetric_difference: どちらか一方の集合には存在するが、両方には ない 値を取得
// 以下の例でこれらをすべて見ていきましょう。

use std::collections::HashSet;

fn main() {
  let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
  let mut b: HashSet<i32> = vec![2i32, 4, 3].into_iter().collect();

  assert!(a.insert(4));
  assert!(a.contains(&4));
  assert!(a.contains(&3));
  assert!(!a.contains(&0));

  // 既に存在する値を追加すると inster() は falseを返す
  // assert!(b.insert(4), "Value 4 is already in set B!"); < -- error

  b.insert(5);

  // If a collection's element type implements `Debug`,
  // then the collection implements `Debug`.
  // It usually prints its elements in the format `[elem1, elem2, ...]`
  // 集合の要素が、`Debug`を実装している型の場合、
  // 集合そのものも`Debug`を実装する。
  // 通常は`[elem1, elem2, ...]`のように要素をプリントする。
  println!("A: {:?}", a);
  println!("B: {:?}", b);

  // Print [1, 2, 3, 4, 5] in arbitrary order
  // [1, 2, 3, 4, 5]を順不同にプリント
  println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

  // This should print [1]
  // これは[1]をプリント
  println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

  // Print [2, 3, 4] in arbitrary order.
  // [2, 3, 4]を順不同にプリント
  println!(
    "Intersection: {:?}",
    a.intersection(&b).collect::<Vec<&i32>>()
  );

  // Print [1, 5]
  // [1, 5]をプリント
  println!(
    "Symmetric Difference: {:?}",
    a.symmetric_difference(&b).collect::<Vec<&i32>>()
  );
}

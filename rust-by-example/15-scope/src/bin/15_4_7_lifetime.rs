// ライフタイム境界
// ジェネリック型に境界(bound)を与え、特定のトレイトの実装を保証するのと同じく
// ライフタイム（それ自身がジェネリック型）にも境界を与えられる
// - 「:」は異なる意味を持つが
// - 「+」はおなじ
// 構文の意味は次の通り
// - T: 'a ： T内の”全ての”参照は'aより長生きでなくてはならない
// - T: Trait+'a ： 上に加えてTはTraitという名のトレイトを実装する必要がある

// 例

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Refは'aというライフタイムを持つジェネリック型Tに対する参照を持つ
// Tの値 "に対する参照" は必ず'aよりも長生きではなくてはならない
// さらにRefのライフタイムは'aを超えてはならない

fn print<T>(t: T)
where
  T: Debug,
{
  println!("`print`: t is {:?}", t);
}

// `Debug`を実装している`T`への参照を取る。`T`への *参照* は
// 必ず`'a`よりも長生きでなくてはならない。さらに、`'a`は
// 関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T)
where
  T: Debug + 'a,
{
  println!("`print_ref`: t is {:?}", t);
}

fn main() {
  let x = 7;
  let ref_x = Ref(&x);

  print_ref(&ref_x);
  print(ref_x);
}

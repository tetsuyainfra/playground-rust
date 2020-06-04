// where句
// トレイト境界は {} の直前にwhere句を導入することでも設けることができる
// where句はパラメータだけでなく任意の型に対してのみ適用できる

// where句が有用なケース
// 例1. ジェネリック型とジェネリック境界に別々に制限を加えたほうが明瞭になる場合
// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// // 下にした方がわかりやすい
// impl<A, D> MyTrait<A, D> for YourType
// where
//   A: TraitB + TraitC,
//   B: TraitE + TraitF,
// {
// }
// 例2.where句の方が通常の構文より表現力が高い場合がある
// (らしけどここではその例は提示していない)

use std::fmt::Debug;

trait PrintInOption {
  fn print_in_option(self);
}

// where句を使わない場合、以下と等価な機能を実装するには
// <T: Debug>という形で表現するか、他の直接的でない方法を使う必要がある
impl<T> PrintInOption for T
where
  Option<T>: Debug,
{
  fn print_in_option(self) {
    // println!("{:?}", self); // これだとエラーになる
    // TがDebugを実装しているとは限らないから？
    println!("{:?}", Some(self));
  }
}

fn main() {
  let vec = vec![1, 2, 3];

  vec.print_in_option();

  // 自分で追加した文
  let vec2 = vec![1, 2, 3];
  vec2.debug_in_option();

  //
  let _vtest = Vec::<f64>::new();
}

// 追加
trait DebugInOption {
  fn debug_in_option(&self);
}

impl<T: Debug> DebugInOption for T {
  // fn debug_in_option(&self) { // でもよい
  fn debug_in_option(self: &T) {
    println!("debug_in-> {:?}", self)
  }
}

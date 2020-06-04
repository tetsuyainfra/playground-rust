// 幽霊型パラメータ
// 幽霊型（PhantomType）は実行時には存在しないが、コンパイル時に静的型チェックされるような型のこと

// 構造体などのデータ型はジェネリック型パラメータを一つ余分に持ち、それをマーカーとして使ったり、コンパイル時の型検査に使える
// マーカーは実際の値を何も持たず、実行時の挙動に影響を与えない

// 以下の例ではPhantomDataを用いて異なった型の値を持つタプルを作成する

use std::marker::PhantomData;

// ジェネリック型タプル構造体 2つ目のパラメータは幽霊型
// PartialEqで比較演算子(==)を使用可能にする
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
  first: A,
  phantom: PhantomData<B>,
}

// ジェネリック型Aに対してはメモリが割り当てられるがBには割り当てられない
// ※おそらくPhantomDataでは型パラメータだけ受け取ってフィールド定義していない

fn main() {
  // <char, f32>
  let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
  // <char, f64>
  let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
  // PhantomDataはいかなる浮動小数点でもない

  // <char, f32>の型が与えられた構造体を作成
  let _struct1: PhantomStruct<char, f32> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };
  // 同様に<char, f64>の構造体
  let _struct2: PhantomStruct<char, f64> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };

  // Compile-time Error! Type mismatch so these cannot be compared:
  // コンパイルエラー！型が違うので`==`で比較することができない！
  // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2); // Error

  // Compile-time Error! Type mismatch so these cannot be compared:
  // コンパイルエラー! 型が違うので比較することができない!
  // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2); // Error
}

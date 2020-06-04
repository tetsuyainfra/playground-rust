// method
// implでメソッドを実装する際にもジェネリック型特有の記法が必要

// サンプル
#[allow(dead_code)]
mod my_temp {
  struct S; // 具象型

  struct GenericVal<T>(T); // ジェネリック型

  // 型パラメータ(f32)を指定した上でメソッドを定義
  impl GenericVal<f32> {}

  // 型パラメータ(上の具象型S）を指定した上でメソッドを定義
  impl GenericVal<S> {}

  // ジェネリック型のまま扱うには<T>が先に来る
  impl<T> GenericVal<T> {}
  // MEMO
  // ここにきてTが利用される前に<T>型パラメータが指定される必要があるの意味が出てきた
}

// 具象型
struct Val {
  val: f64,
}

// ジェネリック型
struct GenVal<T> {
  gen_val: T,
}

// Valに対してimpl
impl Val {
  fn value(&self) -> &f64 {
    &self.val
  }
}

// ジェネリック型Tの場合のメソッドをGenValに対して実装
impl<T> GenVal<T> {
  fn value(&self) -> &T {
    &self.gen_val
  }
}

fn main() {
  let x = Val { val: 3.0 };
  let y = GenVal { gen_val: 3i32 };

  println!("{:?}, {:?}", x.value(), y.value());

  let z = GenVal {
    gen_val: usize::MAX,
  };
  let w = GenVal {
    gen_val: isize::MIN,
  };

  println!("{:?}, {:?}", z.value(), w.value());
}

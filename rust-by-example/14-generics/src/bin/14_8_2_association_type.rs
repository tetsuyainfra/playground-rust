// 関連型
// 関連型を使用すると
// コンテナ型の中の要素をトレイトの中に出力型として書いて全体の可読性をあげられる
// トレイトを定義する際の構文は以下のようになります。

// trait Contains {
//   type A;
//   type B;
//   fn contains(&self, &Self::A, &Self::B)-> bool;
// }
// 関連型を使用しない場合(上の定義では使えない)
// fn difference<A, B, C>(container: &C) -> i32 where
//     C: Contains<A, B> { ... }
// このように書き換えられる
// fn difference<C: Contains>(container: &C) -> i32 {
//   container.last() - container.first()
// }
// これでもOKか・・・
// fn difference<C>(container: &C) -> i32
// where
//   C: Contains,
// {
//   container.last() - container.first()
// }

// 14.8.1 を書き換えると・・・

// 2つの要素がコンテナ型の中に保持されていることを確認するトレイト。
// また、最初あるいは最後の要素を取り出すこともできる。
trait Contains {
  type A;
  type B;
  fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
  fn first(&self) -> i32; // ここは明示しておかないと difference内の引き算で型が確定できない
  fn last(&self) -> i32;
}

struct Container(i32, i32);

impl Contains for Container {
  type A = i32;
  type B = i32;

  fn contains(&self, number_1: &i32, number_2: &Self::B) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }
  fn first(&self) -> i32 {
    self.0
  }
  fn last(&self) -> i32 {
    self.1
  }
}

fn difference<C: Contains>(container: &C) -> i32 {
  container.last() - container.first()
}

fn main() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!(
    "Does container contain {} and {}: {}",
    &number_1,
    &number_2,
    container.contains(&number_1, &number_2)
  );
  println!("First number: {}", container.first());
  println!("Last number: {}", container.last());

  println!("The difference is: {}", difference(&container));
}

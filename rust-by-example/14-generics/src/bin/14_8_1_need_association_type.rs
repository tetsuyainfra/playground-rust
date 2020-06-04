// 関連型
// 関連要素(Associated Items)とは複数の型の要素(items)に関係のある規則の総称
// トレイトの拡張機能で、トレイトの中で新しい要素を定義できる
// そのように定義する要素の一つに関連型がある。
// ジェネリックなコンテナ型に対するトレイトを使用する際、シンプルに書ける

// 関連型が必要になる状況
// コンテナ型にその要素に対してジェネリックなトレイとを実装した場合
// そのトレイトを使用する者には全てのジェネリック型を明記しないといけない

// 以下の例ではContainsトレイトはジェネリック型A,Bを使用している
// その後、Container型に対してContainsを実装している
// その際、fn difference()が使用できるようにA,Bはそれぞれi32と明記されている

// Containsはジェネリックトレイトなのでfn difference()では”全ての”ジェネリック型を宣言しないといけない
// 実際の所、A,Bは引数Cで決定されていて欲しい。
// これは次のページの関連型の機能で解決できる

struct Container(i32, i32);

trait Contains<A, B> {
  fn contains(&self, _: &A, _: &B) -> bool; // A,Bの両方を明示的に要求
  fn first(&self) -> i32; // A,B いずれも要求しない
  fn last(&self) -> i32; // 上におなじ
}

impl Contains<i32, i32> for Container {
  // コンテナ内の2つの要素が等しければTrueを返す
  fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }
  fn first(&self) -> i32 {
    self.0
  }
  fn last(&self) -> i32 {
    self.1
  }
}

// `A`と`B`は`C`に保持されていることを考慮すると、`A`と`B`を
// ２度も書くのは面倒
fn difference<A, B, C>(container: &C) -> i32
where
  C: Contains<A, B>, // ここで明示されている
{
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

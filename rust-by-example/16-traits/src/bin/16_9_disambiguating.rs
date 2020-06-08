// Disambiguating overlapping traits
// 重複したトレイトの明確化(disambiguating?)

// 型は多くの異なる特性を実装できる
// 2つのトレイとの両方に同じ名前があった場合どうするか
// 例えば get() は多くのトレイトに存在する
// 戻り値の型が異なる場合もある

// 各トレイとの実装は独自のimplブロックで定義されるのでどのトレイトのgetを実装するかは明らか

// 呼び出す時に明確にするために完全修飾構文を使用する

trait UsernameWidget {
  // Get the selected username out of this widget
  fn get(&self) -> String;
}

trait AgeWidget {
  // Get the selected age out of this widget
  fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
  username: String,
  age: u8,
}

impl UsernameWidget for Form {
  fn get(&self) -> String {
    self.username.clone()
  }
}

impl AgeWidget for Form {
  fn get(&self) -> u8 {
    self.age
  }
}

fn main() {
  let form = Form {
    username: "rustacean".to_owned(),
    age: 28,
  };

  // UsernameWidget, AgeWidgetどっちのgetを呼ぶかわからない
  // println!("{}", form.get())

  let username = <Form as UsernameWidget>::get(&form);
  assert_eq!("rustacean", username);

  let age = <Form as AgeWidget>::get(&form);
  assert_eq!(28, age);
}

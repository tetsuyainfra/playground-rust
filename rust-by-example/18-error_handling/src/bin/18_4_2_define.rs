// エラータイプの定義
// 場合によっては、コードを単純化して、さまざまなエラーをすべて1つのタイプのエラーでマスクします。
// これをカスタムエラーで示します。

// Rustでは、独自のエラータイプを定義できます。 一般的に、「良い」エラータイプの例
// - 同じタイプの異なるエラーを表している
// - ユーザーに素敵なエラーメッセージを表示する
// - 他のタイプと比較するのが簡単
//   Good：Err(EmptyVec)
//   Bad: Err("少なくとも1つの要素を持つベクトルを使用してください" .to_owned())
// - エラーに関する情報を保持できます
//   Good：Err(BadChar(c, position))
//   Bad：Err("+はここでは使用できません" .to_owned())
// - 他のエラーとうまく合成(Compose)できる

use std::error;
use std::fmt;

//エラータイプを定義します。 これらは、エラー処理のケースに合わせてカスタマイズできます。
//これで、独自のエラーを記述して、根本的なエラーを遅らせることができます
//実装、またはその間に何かを行います。
#[derive(Debug, Clone)]
struct DoubleError;

// ジェネリック型のエイリアスを作成
type Result<T> = std::result::Result<T, DoubleError>;

// エラーの生成は、エラーの表示方法とは完全に切り離されています。
// 複雑なロジックが表示スタイルで乱雑になることを心配する必要はありません。
//
// エラーに関する追加情報は保存しないことに注意してください。
// これは、情報を伝達するようにタイプを変更しないと、解析に失敗した文字列を示すことができないことを意味します。
impl fmt::Display for DoubleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid first item to double")
  }
}

impl error::Error for DoubleError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
  vec
    .first()
    // 新しく定義したエラー型に変更する
    .ok_or(DoubleError)
    .and_then(|s| {
      s.parse::<i32>()
        // ここも同じく、変更する
        .map_err(|_| DoubleError)
        .map(|i| 2 * i)
    })
}

fn print(result: Result<i32>) {
  match result {
    Ok(n) => println!("The first doubled is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}

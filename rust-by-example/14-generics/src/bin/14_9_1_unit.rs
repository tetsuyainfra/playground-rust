// testcase: 単位を扱う
// 共通の単位同士を扱う際のチェックのために Addを幽霊型を用いた実装にすると便利なことがある
// Add は 加算演算子をつかうためのTrait
/* このように定義しておくと Self + RHS = Outputが保証される
   かつ、impl時にRHS型が明示されてないとデフォルトでSelfと同じになる
pub trait Add<RHS = Self> {
  type Output;
  fn add(self, rhs: RHS) -> Self::Output;
}
impl <U> Add for T<U> {
  type Output = T<U>;
  ...
}
*/

// 実例

use std::marker::PhantomData;
use std::ops::Add;

/// Create void enumerations to define unit types.
/// 単位を定義するため、空の列挙型を作成。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length`は`Unit`という幽霊型パラメータを持つ型
/// `f64`ははじめから`Clone`、`Copy`トレイトを持っている。
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
  type Output = Length<Unit>;
  fn add(self, rhs: Self) -> Self::Output {
    Length(self.0 + rhs.0, PhantomData)
  }
}

fn main() {
  // `one_foot`が幽霊型`Inch`を持つように明示する。
  let one_foot: Length<Inch> = Length(12.0, PhantomData);
  // `one_meter`が幽霊型`Mm`を持つように明示する。
  let one_meter: Length<Mm> = Length(1000.0, PhantomData);

  // それぞれ、上記で定義したaddが呼ばれる
  // Lengthはコピートレイトを持っているので self,rhsはコピーされたものが渡る
  let two_feet = one_foot + one_foot;
  let two_meters = one_meter + one_meter;
  // 加算が問題なく実行されていることを確認

  println!("one foot + one_foot = {:?} in", two_feet.0);
  println!("one meter + one_meter = {:?} mm", two_meters.0);

  // これはもちろんえらー
  // let one_feter = one_foot + one_meter; // Error
}

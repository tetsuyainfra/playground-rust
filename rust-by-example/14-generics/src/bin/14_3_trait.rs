// Generic Trait
// トレイトもジェネリクスを活用できる
// Dropトレイとをジェネリックメソッドとして再実装して自身と引数として受け取った値の両方をdropするようなメソッドにする

// コピー不可能な型を作る
// ※Cloneメソッドを持ちいない限り、値のコピーではなくムーブが起きる型
// ※Copy Traitを実装していないからそうなると思われる
struct Empty;
struct Null;

trait DoubleDrop<T> {
  // selfに加えてもう一つのジェネリック型を受け取り、
  // 何もしないメソッドのシグネチャを定義
  fn double_drop(self, _: T);
}

// Uをself,Tをもう一つの引数としてして受け取るジェネリックトレイトを実装する
// U,Tは型パラメータでジェネリック型となる
impl<T, U> DoubleDrop<T> for U {
  // このメソッドを呼ぶと束縛変数がmoveされて関数が終わるとともに解放(drop)される
  fn double_drop(self, _: T) {}
}

fn main() {
  let empty = Empty;
  let null = Null;

  empty.double_drop(null);

  // 上の行でmove・解放されてるので使えない
  // empty;
  // null;
}

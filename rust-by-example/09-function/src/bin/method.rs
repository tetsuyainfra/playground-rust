struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  fn new(x: f64, y: f64) -> Point {
    Point { x, y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  fn area(&self) -> f64 {
    // `self`はドット演算子によって構造体のfieldを参照できる。
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
    ((x1 - x2) * (y1 - y2)).abs()
  }

  //
  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // This method requires the caller object to be mutable
  // `&mut self` desugars to `self: &mut Self`
  // このメソッドは呼び出し元オブジェクトがミュータブルであることを
  // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

// Boxはヒープ上で値を確保する
struct Pair(Box<i32>, Box<i32>);

impl Pair {
  fn destroy(self) {
    let Pair(first, second) = self;
    println!("Destroying Pair({}, {})", first, second);
  }
}

fn main() {
  let rectangle = Rectangle {
    // Static methods are called using double colons
    // スタティックメソッドはコロンを2つ挟んで呼び出される。
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  // インスタンスメソッドはドット演算子を用いて呼び出される。
  // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
  // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
  // 必要とする。
  // rectangle.translate(1.0, 0.0);
  // TODO ^ この行をアンコメントしてみましょう。 --> error

  // Okay! Mutable objects can call mutable methods
  // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
  square.translate(1.0, 1.0);

  let pair = Pair(Box::new(1), Box::new(2));

  pair.destroy();

  // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
  // pair.destroy();
  // TODO ^ この行をアンコメントしてみましょう。 --> error
}

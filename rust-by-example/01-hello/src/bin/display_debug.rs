use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x:{}, y:{}", self.x, self.y)
  }
}

fn main() {
  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax); // Display: (0, 14)
  println!("Debug: {:?}", minmax); // Debug: MimMax(0, 14)

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!(
    "The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range
  );

  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  // Error. Both `Debug` and `Display` were implemented, but `{:b}`
  // requires `fmt::Binary` to be implemented. This will not work.
  // `Debug`と`Display`は実装されていますが、`fmt::Binary`はされていないため
  // `{:b}`使用している以下の例はエラーになります、
  // println!("What does Point2D look like in binary: {:b}?", point);

  // 演習
  #[derive(Debug)]
  struct Complex {
    real: f64,
    imag: f64,
  }

  impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} + {}i", self.real, self.imag)
    }
  }

  let complex = Complex {
    real: 3.3,
    imag: 7.2,
  };

  println!("Compare complex:");
  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);
}

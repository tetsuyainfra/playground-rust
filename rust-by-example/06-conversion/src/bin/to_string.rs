use std::fmt;

struct Circle {
  radius: i32,
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of raidus {}", self.radius)
  }
}

fn main() {
  let circle = Circle { radius: 8 };
  // fmt::Display implを実装していれば .to_string()も自動で提供される
  println!("{}", circle.to_string());

  // FromStr trait で実装されていてstdライブラリに含まれている
  let parsed: i32 = "5".parse().unwrap();
  // "turbofish" 構文で呼び出す
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println!("Sum: {}", sum);
}

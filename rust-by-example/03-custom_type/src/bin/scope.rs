

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main () {
  let n = 16;

    // Access constant in the main thread
    // main 関数の中から定数を参照
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // エラー!`const`は変更できません。
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう
}
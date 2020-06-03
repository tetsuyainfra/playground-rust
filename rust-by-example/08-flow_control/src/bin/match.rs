// match対象の値
// 単一の値
// 複数の値
// 範囲
// その他

// matchは全ての可能な値をカバーしなくてはならない
// ※明示的に _ => {} を書く必要がある

fn main() {
  let number = 13;
  // TODO ^ Try different values for `number`

  println!("Tell me about {}", number);
  match number {
    // Match a single value
    // 単一の値とのマッチをチェック
    1 => println!("One!"),
    // Match several values
    // いくつかの値とのマッチをチェック
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    // Match an inclusive range
    // 特定の範囲の値とのマッチをチェック
    13..=19 => println!("A teen"),
    // Handle the rest of cases
    // その他の場合の処理
    _ => println!("Ain't special"),
  }

  let boolean = true;
  // Match is an expression too
  // マッチは式でもある。
  let binary = match boolean {
    // The arms of a match must cover all the possible values
    // マッチは全ての可能な値をカバーしなくてはならない
    false => 0,
    true => 1,
    // TODO ^ Try commenting out one of these arms
    // TODO ^ 試しに片方をコメントアウトしてみましょう。 -> コンパイルエラー
  };

  println!("{} -> {}", boolean, binary);
}

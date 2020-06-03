// if let は これが一つの式って考えたほうがよさそう

// let optional = Some(7);
// match optional {
//     Some(i) => {
//         println!("This is a really long string and `{:?}`", i);
//         // ^ `i`をoption型からデストラクトするためだけに
//         // インデントが一つ増えてしまっている。
//     },
//     _ => {},
//     // ^ `match`は全ての型に対して網羅的でなくてはならないので必要。
//     // 冗長に見えませんか？
// };

fn main() {
  // All have type `Option<i32>`
  // 全て`Option<i32>`型
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  // The `if let` construct reads: "if `let` destructures `number` into
  // `Some(i)`, evaluate the block (`{}`).
  // `if let`文は以下と同じ意味.
  //
  // もしletがnumberをデストラクトした結果が`Some(i)`になるならば
  // ブロック内(`{}`)を実行する。
  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  // If you need to specify a failure, use an else:
  // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、
  // `else`を使用する。
  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else {
    // Destructure failed. Change to the failure case.
    // デストラクト失敗の場合。このブロック内を実行
    println!("Didn't match a number. Let's go with a letter!");
  }

  // Provide an altered failing condition.
  // デストラクト失敗時の処理を更に分岐させることもできる
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  // Destructure failed. Evaluate an `else if` condition to see if the
  // alternate failure branch should be taken:
  // デストラクト失敗。`else if`を評価し、処理をさらに分岐させる。
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter!");
  } else {
    // The condition evaluated false. This branch is the default:
    // 今回は`else if`の評価がfalseなので、このブロック内がデフォルト
    println!("I don't like letters. Let's go with an emoticon :)!");
  }

  let_like_match()
}

enum Foo {
  Bar,
  Baz,
  Qux(u32),
}

fn let_like_match() {
  // Create example variables
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  // Variable a matches Foo::Bar
  if let Foo::Bar = a {
    println!("a is foobar");
  }

  // Variable b does not match Foo::Bar
  // So this will print nothing
  if let Foo::Bar = b {
    println!("b is foobar");
  }

  // Variable c matches Foo::Qux which has a value
  // Similar to Some() in the previous example
  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  // Binding also works with `if let`
  if let Foo::Qux(value @ 100) = c {
    println!("c is one hundred");
  }

  // if let はパラメータの値になってないenum列挙型でもマッチできる？
  enum Foo2 {
    Bar,
  };
  let a = Foo2::Bar;
  // if Foo2::Bar == a は 失敗する
  // なぜならインスタンスを同等化できないため
  // if Foo2::Bar == a {
  if let Foo2::Bar = a {
    // ^-- this causes a compile-time error. Use `if let` instead.
    println!("a is foobar");
  }

  match a {
    Foo2::Bar => {
      println!("match a is foobar");
    }
  }
}

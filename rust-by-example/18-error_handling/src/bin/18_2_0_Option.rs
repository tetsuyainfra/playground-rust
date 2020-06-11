// 最後の例では、プログラムの失敗を自由に誘導できることを示しました。
// 私たちはpanic、王女が不適切な贈り物、つまり蛇を受け取ったかどうかをプログラムに伝えました。
// しかし、王女が贈り物を期待していてもらえなかった場合はどうなりますか？
// その場合も同様に悪いので、処理する必要があります！

//ヘビで行うように、ヌル文字列（）に対してこれをテストできます。
//Rustを使用しているので、代わりに贈り物がないケースをコンパイラーに指摘させましょう。

// enum呼ばれるOption<T>にstd不在可能性があるときにライブラリが使用されています。
//次の2つの「オプション」のうちの1つとして現れます。
// - Some(T): 型Tの値がある場合
// - None: 値が存在しない場合。
// これらはmatchを用いて明示的に扱うこともできますし、unwrapで暗黙に処理することもできます。後者はSomeの中の値を返すかpanicするかのどちらかです。

// expectメソッドを用いて、panicを手動でカスタマイズできることに触れておきましょう。
// これは（unwrapをそのまま用いた場合よりも）内容が理解しやすいエラーメッセージを出力するのに役立ちます。
// 次の例では、結果をより明示的に、可能ならいつでもpanicできるように扱っていきます。

fn give_commoner(gift: Option<&str>) {
  match gift {
    Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
    Some(inner) => println!("{}? How nice.", inner),
    None => println!("No gift? Oh well."),
  }
}

fn give_princess(gift: Option<&str>) {
  // `unwrap`を使用すると値が`None`だった際に`panic`を返します。。
  let inside = gift.unwrap();

  if inside == "snake" {
    panic!("AAAaaaa!!!");
  }

  println!("I love {}s !!!!!", inside);
}

fn main() {
  let food = Some("cabbage");
  let snake = Some("snake");
  let void = None;

  give_commoner(food);
  give_commoner(snake);
  give_commoner(void);

  let bird = Some("robin");
  let nothing: Option<&str> = None;

  give_princess(bird);
  give_princess(nothing); // unwrap()されるときにpanicを返す

  // add
  give_princess(Some("snake")); // unwrap()されるときにpanicを返す
}

fn use_match() {
  // Make `optional` of type `Option<i32>`
  // `Option<i32>`の`optional`を作成
  let mut optional = Some(0);

  // Repeatedly try this test.
  // 変数の照合を繰り返し行う。
  loop {
    match optional {
      // If `optional` destructures, evaluate the block.
      // もし`optional`のデストラクトに成功した場合、値に応じて処理を分岐
      Some(i) => {
        if i > 9 {
          println!("Greater than 9, quit!");
          optional = None;
        } else {
          println!("`i` is `{:?}`. Try again.", i);
          optional = Some(i + 1);
        }
        // ^ Requires 3 indentations!
        // ^ 3つものインデントが必要。
      }
      // Quit the loop when the destructure fails:
      // デストラクトに失敗した場合、ループを脱出
      _ => {
        break;
      } // ^ Why should this be required? There must be a better way!
        // どうしてこんな行を書く必要が?もっと良い方法があるはずです!
    }
  }
}

// while letを使う場合
// インデントが少なくデストラクト失敗時の処理を追加で書く必要がない
fn use_while_let() {
  let mut optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("`i` is `{:?}`. Try again.", i);
      optional = Some(i + 1);
    }
  }
}

fn main() {
  use_match();
  use_while_let()
}

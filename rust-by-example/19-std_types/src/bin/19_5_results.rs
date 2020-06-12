// Result
// これまでの例で、失敗する可能性のある関数の返り値として、
// 列挙型Optionが使用でき、失敗時の返り値にはNoneを用いることを見てきました。
// しかし、時には なぜ そのオペレーションが失敗したのかを明示することが重要な場合があります。
// そのためにはResult列挙型を使用します。

// 列挙型Result<T, E>は2つの値をとりえます。
// - Ok(value) ... これはオペレーションが成功したことを意味し、返り値valueをラップします。（valueは型Tを持ちます。）
// - Err(why) ... これはオペレーションの失敗を意味します。whyをラップしており、ここには失敗した理由が（必ずではありませんが）書かれています。（whyの型はEです。）

// 追加 自分的に演習としてf32/f64の2種類の関数を定義してみる
// コードの共通化には2つの戦略がある
// 1. ジェネリクスを駆使する
// 2. マクロで実装するパターン
// div()関数はジェネリクスを使ってみる
// マクロのほうが直感的であるが・・・

mod checked {
  #[derive(Debug)]
  pub enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
  }

  pub type MathResult = Result<f64, MathError>;
  // pub type MathResult<T> = Result<T, MathError>; // f32/f64に対応させるのはこれはむずいぞｗｗｗ
  // num-traitsを使えば良さそうだけども・・・stdlibではきついっす

  pub fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
      // This operation would `fail`, instead let's return the reason of
      // the failure wrapped in `Err`
      // 分母が0なので、このオペレーションは普通に行えば失敗する。
      // 代わりに`Err`でラップされた失敗の理由を返そう。
      Err(MathError::DivisionByZero)
    } else {
      // This operation is valid, return the result wrapped in `Ok`
      // このオペレーションは問題がないので、結果を`Ok`でラップして返そう。
      Ok(x / y)
    }
  }

  pub fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
      Err(MathError::NegativeSquareRoot)
    } else {
      Ok(x.sqrt())
    }
  }

  pub fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
      Err(MathError::NonPositiveLogarithm)
    } else {
      Ok(x.ln())
    }
  }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
  // This is a three level match pyramid!
  // 3段階の`match`ピラミッド！
  match checked::div(x, y) {
    Err(why) => panic!("{:?}", why),
    Ok(ratio) => match checked::ln(ratio) {
      Err(why) => panic!("{:?}", why),
      Ok(ln) => match checked::sqrt(ln) {
        Err(why) => panic!("{:?}", why),
        Ok(sqrt) => sqrt,
      },
    },
  }
}

fn main() {
  // Will this fail?
  // これは失敗するだろうか？
  println!("{}", op(1.0, 10.0));
}

// Domain Specific Languages(DSLs)
// DSL はRustマクロに埋め込まれた小さな言語です(Rustマクロで書くことができる小さな組み込み言語ぐらいのほうがいいかも)
// マクロシステムは通常のRustコンストラクトに拡張されるため、
// 完全に有効なRustですが、小さな言語のように見えます。
// これにより、いくつかの特別な機能（境界内）の簡潔なまたは直感的な構文を定義できます。

// 小さな電卓APIを定義したいとします。
// 式を指定して、出力をコンソールに出力したいと思います。
// これは小さな例ですが、lazy_static, clap など遥かに複雑なインターフェイスが開発さています

macro_rules! calculate {
  // ↓ここでevalが定義されてる
  (eval $e: expr) => {{ // <--ブレースが 2組みに注意
    {                   // <--
      let val: usize = $e; // Force Type to be integers
      println!("{} = {}", stringify!{$e}, val);
    }
  }};
}

fn main() {
  calculate! {
    eval 1 + 2 // evalはRustのキーワードではない
  }

  calculate! {
    eval (1+2) * (3/4)
  }
}

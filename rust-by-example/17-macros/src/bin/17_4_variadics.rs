// Variadic Interfaces 可変インターフェイス
// 任意の数の引数を取る。
// println!などフォーマット文字列によって決定されるように任意の数の引数を取ることができる

// 17_3より
macro_rules! calculate {
  (eval $e: expr) => {{ // <--ブレースが 2組みに注意
    {                   // <--
      let val: usize = $e; // Force Type to be integers
      println!("{} = {}", stringify!{$e}, val);
    }
  }};

  (eval $e:expr, $(eval $es: expr), +) => {{
    calculate! { eval $e }
    calculate! { $(eval $es), + }
  }}
}

fn main() {
  calculate! {
    eval 1+2,
    eval 3+4,
    eval (2*3)+1
  }
}

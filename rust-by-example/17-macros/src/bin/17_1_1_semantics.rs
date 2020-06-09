// 識別子
// macroの引数は $ がつく
// 型は識別子で(designator)でアノテーションされる
// ($変数名: 型) という形

// 使える識別子一覧
// block
// expr 式に使用
// ident 関数、変数の名前に使用
// item
// literal is used for literal constants
// pat (pattern)
// path
// stmt (statement)
// tt (token tree)
// ty (type)
// vis (visibility qualifier)

macro_rules! create_function {
  // このマクロは`ident`識別子に対する値を引数として鳥
  // $func_name という名の関数を作成する
  // `ident`識別子は関数・変数の名前用の識別子
  ($func_name: ident) => {
    fn $func_name() {
      // ※ strinfify! マクロはidentを文字列に変える
      println!("You called {:?}()", stringify!($func_name));
    }
  };
}

// マクロの利用
create_function!(foo);
create_function!(bar);

// このマクロは`expr`識別子に対応する値を引数として取り、
// その結果を文字列としてプリントする。
// `expr`識別子は式に対応する。
macro_rules! print_result {
  ($expression: expr) => {
    // `stringify!`は式を *そのままの形で* 文字列に変換する
    println!("{:?} = {:?}", stringify!($expression), $expression)
  };
}

fn main() {
  foo();
  bar();

  print_result!(1u32 + 1);

  // ブロックも式の一種である
  print_result!({
    let x = 1u32;

    x * x + 2 * x - 1
  });
}

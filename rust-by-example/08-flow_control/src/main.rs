//
// if-elseは式の一種
//     if-elseの分岐先の返り値の型は同一でないといけない
//
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        // この式は`i32`を返す。
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        // ここでも返り値の型は`i32`でなくてはならない。
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
        // TODO ^ セミコロン(`;`)をつけて、返り値を返さないようにしてみましょう
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.
    //   ここにセミコロンを付けるのを忘れないように!
    //   `let`による変数束縛の際には必ず必要です!

    println!("{} -> {}", n, big_n);

    // 実験
    // そのもそもif文が束縛しなければ、値を返すとエラーになる
    if n < 0 {
        print!("{} is negative", n);
    // 1
    } else if n > 0 {
        print!("{} is positive", n);
    // 2
    } else {
        print!("{} is zero", n);
        // 3
    }

    // 実験
    let big_n = if n < 10 && n > -10 {
        //
        10 * n
    } else {
        //
        n / 2
    }; // ; が無いとコンパイルエラー
       // println!("{} -> {}", n, big_n);
}

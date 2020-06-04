fn function() {
  println!("called `function()`");
}

mod cool {
  pub fn function() {
    println!("called `cool::function()`");
  }
}

mod my {
  fn function() {
    println!("called `my::function()`");
  }

  mod cool {
    pub fn function() {
      println!("called `my::cool::function()`");
    }
  }

  pub fn indirect_call() {
    // Let's access all the functions named `function` from this scope!
    // `function`という名の様々な関数をこのスコープ内から参照してみましょう。
    print!("called `my::indirect_call()`, that\n> ");

    // The `self` keyword refers to the current module scope - in this case `my`.
    // Calling `self::function()` and calling `function()` directly both give
    // the same result, because they refer to the same function.
    // `self`キーワードは現在のモジュールスコープを示す。この場合は`my`。
    // `self::function()`と`funcition()`は同じ関数であるため、同じ結果になる。
    self::function(); // -> my::function()
    function(); // -> my::functionが呼ばれるがわかりにくい

    // We can also use `self` to access another module inside `my`:
    // `my`以下の別のモジュールを呼び出す際に`self`を用いて明示的に参照できる。
    self::cool::function(); // -> 別階層ならselfを付けてよべる
    cool::function(); // -> これでも伝わるが・・・同名crateがあればわかりにくくなりそう

    // The `super` keyword refers to the parent scope (outside the `my` module).
    // `super`は親スコープ（`my`の外側）を参照する。
    super::function(); // -> global::function()が呼ばれた

    // This will bind to the `cool::function` in the *crate* scope.
    // In this case the crate scope is the outermost scope.
    // 以下は *クレート* スコープ内の`cool::function`をバインディングする。
    // この場合、クレートスコープは一番外側のスコープである。
    {
      use crate::cool::function as root_function;
      use crate::cool::function;
      root_function();
      function(); // もちろん cool::function()
    }
  }
}

fn main() {
  my::indirect_call();
}

// 通常はクレートを使うと宣言する
// extern crate deeply; // normally, this would exist and not be commented out!
// use crate::deeply::nested::{my_first_function, my_second_function, AndATraitType};
//
// fn main() {
//   my_first_function();
// }

// Bind the `deeply::nested::function` path to `other_function`.
// `deeply::nested::function`を`other_function`にバインド
use deeply::nested::function as other_function;

fn function() {
  println!("called `function()`");
}

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function()`");
    }
  }
}

fn main() {
  other_function(); // --> nested::function()のエイリアス

  println!("Entering block");
  {
    // これは`use deeply::nested::function as function`と同等
    // この`function()`は外の`function()`をシャドウイングする
    use crate::deeply::nested::function;
    function(); // --> nested::function()

    // `use`バインディングは局所的なスコープを持つ。
    // この場合には`function()`のシャドウイングはこのブロック内のみ
    println!("Leaving block");
  }

  function(); // --> ::function()
}

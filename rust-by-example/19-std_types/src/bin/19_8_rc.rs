// Rc 参照カウンタ
// When multiple ownership is needed, Rc(Reference Counting) can be used.
// 複数の所有権が必要な時Rcが使われます
// Rc keeps track of the number of the references
// Rc は参照数を追跡記録します
// which means the number of owners of the value wrapped inside an Rc.
// 参照数は値の所有者数を示し、Rc内に保存される

// Reference count of an Rc increases by 1 whenever an Rc is cloned,
// Rcの参照カウンタはRcが1つクローンされるたびに1づつ増え、
// and decreases by 1 whenever one cloned Rc is dropped out of the scope.
// クローンされたRCがスコープから外れるたびに１づつ減る
// When an Rc's reference count becomes zero,
// Rcの参照カウンタがゼロになった時
// which means there are no owners remained, both the Rc and the value are all dropped.
// それは記憶している所有者がいなくなった事で、Rcと値がすべてdoropされる

// Cloning an Rc never do a deep copy.
// Rcのクローンは絶対にディープコピーしない
// Cloning creates just another pointer to the wrapped value, and increments the count.
// クローンでは単に別の（新たな）ラップした値へのポインタが作られ、カウンタが増えるだけである。

use std::rc::Rc;

fn main() {
  let rc_examples = "Rc examples".to_string();
  {
    println!("--- rc_a is created ---");

    let rc_a: Rc<String> = Rc::new(rc_examples);
    println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

    {
      println!("--- rc_a is cloned to rc_b ---");

      let rc_b: Rc<String> = Rc::clone(&rc_a);
      println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
      println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

      // Two `Rc`s are equal if their inner values are equal
      println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

      // We can use methods of a value directly
      println!("Length of the value inside rc_a: {}", rc_a.len());
      println!("Value of rc_b: {}", rc_b);

      println!("--- rc_b is dropped out of scope ---");
    }

    println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

    println!("--- rc_a is dropped out of scope ---");
  }

  // Error! `rc_examples` already moved into `rc_a`
  // And when `rc_a` is dropped, `rc_examples` is dropped together
  // println!("rc_examples: {}", rc_examples); // <-- コンパイル時にエラー
  // TODO ^ Try uncommenting this line
}

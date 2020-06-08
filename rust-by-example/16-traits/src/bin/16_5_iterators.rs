// Iterators
// Iterator Trait は 配列のようなコレクションを繰り返すために埋め込まれている

// このトレイトはnextメソッドの定義を要求する

// for文はinter_iter()を使ってコレクションからイテレータを作る

struct Fibonacchi {
  curr: u32,
  next: u32,
}

impl Iterator for Fibonacchi {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

fn fibonacci() -> Fibonacchi {
  Fibonacchi { curr: 0, next: 1 }
}

fn main() {
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  // `for` works through an `Iterator` until it returns `None`.
  // Each `Some` value is unwrapped and bound to a variable (here, `i`).
  println!("Iterate through 0..3 using `for`");
  for i in 0..3 {
    println!("> {}", i);
  }

  // The `take(n)` method reduces an `Iterator` to its first `n` terms.
  println!("The first four terms of the Fibonacci sequence are: ");
  for i in fibonacci().take(4) {
    println!("> {}", i);
  }

  // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
  println!("The next four terms of the Fibonacci sequence are: ");
  for i in fibonacci().skip(4).take(4) {
    println!("> {}", i);
  }

  let array = [1u32, 3, 3, 7];

  // The `iter` method produces an `Iterator` over an array/slice.
  println!("Iterate the following array {:?}", &array);
  for i in array.iter() {
    println!("> {}", i);
  }
}

#![allow(unreachable_code)]

fn main() {
  'outer: loop {
    println!("Entered the outer loop");

    #[allow(unused_labels)]
    'inner: loop {
      println!("Entered the inner loop");

      // This would break only the inner loop
      // これは内側のループのみを中断します。
      //break;

      // This breaks the outer loop
      // こちらは外側を中断します
      break 'outer;
    }

    println!("This point will never be reached");
  }

  println!("Exited the outer loop");
}

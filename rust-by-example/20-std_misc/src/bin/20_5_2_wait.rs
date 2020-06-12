// dropの延期
// If you'd like to wait for a process::Child to finish,
// 子プロセスの終了を待ちたいとき
//  you must call Child::wait, which will return a process::ExitStatus.
// Child::waitを呼び process::ExitStatusが帰ってくるのを待つ

use std::process::Command;

fn main() {
  let mut child = Command::new("sleep").arg("5").spawn().unwrap();
  let _result = child.wait().unwrap();

  println!("{:?}", _result); // Add
  println!("reached end of main");
}

// $ rustc wait.rs && ./wait
// # `wait` keeps running for 5 seconds until the `sleep 5` command finishes
// reached end of main

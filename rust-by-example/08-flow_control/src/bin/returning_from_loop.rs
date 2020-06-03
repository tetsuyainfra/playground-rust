fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2; // break文はloopの終了と値を返せる
    }
  };

  assert_eq!(result, 20);
}

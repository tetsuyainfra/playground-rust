// 前の例は常に非常に便利でした。 結果は他の結果と相互作用し、オプションは他のオプションと相互作用します。

// OptionがResultと対話する必要がある場合や、
// Result <T、Error1>がResult <T、Error2>と対話する必要がある場合があります。
// そのような場合は、さまざまなエラータイプを、構成可能で対話しやすい方法で管理したいと考えています。

// 次のコードでは、unwrapの2つのインスタンスが異なるエラータイプを生成します。
// Vec :: firstはオプションを返し、parse :: <i32>はResult <i32、ParseIntError>を返します。

fn double_first(vec: Vec<&str>) -> i32 {
  let first = vec.first().unwrap(); // Generate error 1
  2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {}", double_first(numbers));

  println!("The first doubled is {}", double_first(empty));
  // Error 1: the input vector is empty

  println!("The first doubled is {}", double_first(strings));
  // Error 2: the element doesn't parse to a number
}

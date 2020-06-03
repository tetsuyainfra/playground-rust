// クロージャを返す関数
// クロージャを返すことは可能だが、無名のクロージャの型はunknownである
// よって impl traitを返す
// （例によって）Fn, FnMut, FnOnceである

// 値をキャプチャするクロージャ―ではmove keywordは絶対に必要である
// なぜなら無名関数が定義された関数をでるとすぐに参照された変数がdropされるからだ
// (返す)クロージャ内に無効な参照をのこすことはできない
fn create_fn() -> impl Fn() {
  // let text = "Fn".to_owned(); // copy じゃなくて所有を移すやつ
  let text = "Fn"; // べつにこれでもmoveされるから問題ない(クロージャに移す時Copyされるけど)(たぶん)

  move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
  let text = "FnMut".to_owned();

  move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
  let text = "FnOnce".to_owned();

  move || println!("This is a: {}", text)
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  let fn_once = create_fnonce();

  fn_plain();
  fn_mut();
  fn_once();
}

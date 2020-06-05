// 15.3.1 mutability
// ミュータブルなデータ(let mut val)はミュータブルに借用(let ref_val = &mut val)できる
// ミュータブルな参照と呼ばれ、読み込み、書き込みの権限を借用者に与える
// 反対に(&T)はデータをイミュータブルな参照を介して借用し、借用した側はデータを読み込めるが書き込めない
// (let ref_val = &mut val)

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  author: &'static str,
  title: &'static str,
  year: u32,
}

// 参照を取る
fn borrow_book(book: &Book) {
  println!("I immutably borrowd {} - {} edition", book.title, book.year);
}

// 変更可能な参照を取る
fn new_edition(book: &mut Book) {
  book.year = 2014;
  println!("I mutably borrowd {} - {} edition", book.title, book.year);
}

fn main() {
  let immutabook = Book {
    // 「"」で囲まれた部分は`&'static str`型になる。
    author: "Douglas Hofstadter",
    title: "Gödel, Escher, Bach",
    year: 1979,
  };

  // 変更可能なコピーを作成する（Copy Traitのおかげ）
  let mut mutabook = immutabook;

  // 変更不可なオブジェクトを変更不可に借用する
  borrow_book(&immutabook);

  // 変更可なオブジェクトを変更不可に借用する
  borrow_book(&mutabook);

  // 変更可なオブジェクトを変更可で借用する
  new_edition(&mut mutabook);

  // 変更不可なオブジェクトは変更可で借用できない
  // new_edition(&mut immutabook); // Error

  // 借用
  borrow_book(&mutabook); // 値が変わってるのを確認してみる
}

use std::fmt;

// #[warn(dead_code)] // こっちがデフォルトで未使用エラーを出す
#[allow(dead_code)]
struct StructureNo(i32);

// ジェネリックでない型は自由にfmt::Displayを定義してしまえば良い
struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "impl fmt->{}", self.0)
    }
}

fn main() {
    // let x_no = StructureNo(12);
    // println!("Hello, world! {}", x_no); // fmt::Displayトレイトを実装していないのでエラー

    let x = Structure(12);
    println!("x: {}", x); // x: impl fmt->12
}

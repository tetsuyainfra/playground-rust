// 構造体の場合、フィールドごとにpublic/privateを設定できる
// デフォルトではprivateだが、pub宣言でpublicにできる
// 構造体がモジュール外から参照される時に限り意味があり情報の隠蔽化（カプセル化）に意味がある

mod my {
  pub struct OpenBox<T> {
    pub contents: T,
  }

  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T,
  }

  impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox { contents: contents }
    }
  }
}

fn main() {
  let open_box = my::OpenBox {
    contents: "public information",
  };
  // and their fields can be normally accessed.
  // フィールドにも普通にアクセスできる。
  println!("The open box contains: {}", open_box.contents);

  // Public structs with private fields cannot be constructed using field names.
  // Error! `ClosedBox` has private fields
  // プライベートなフィールドを持つ構造体は、インスタンス化する際に
  // フィールド名を指定することができない。
  // エラー!`ClosedBox`にはプライベートな属性が存在します。
  // let closed_box = my::ClosedBox { contents: "classified information" }; // <-- error
  // TODO ^ Try uncommenting this line
  // TODO ^ 試しにここをアンコメントしてみましょう。

  // However, structs with private fields can be created using
  // public constructors
  // そのような場合でも、パブリックなコンストラクタを介して作成
  // することは可能。
  let _closed_box = my::ClosedBox::new("classified information");

  // and the private fields of a public struct cannot be accessed.
  // Error! The `contents` field is private
  // たとえパブリックな構造体でも、プライベートなフィールドには
  // アクセス出来ない。
  // エラー!`contents`フィールドはプライベートです。
  //println!("The closed box contains: {}", _closed_box.contents); <-- error
  // TODO ^ Try uncommenting this line
  // TODO ^ ここをアンコメントしてみましょう。
}

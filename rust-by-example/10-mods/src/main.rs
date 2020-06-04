// 10.5 ファイルの階層構造
// 10.1 は bin/private_public.rsに移動

// このように宣言すると、`my.rs`または、`my/mod.rs`という名のファイルを探し、
// その内容をこのファイル中で`my`という名から使用することができるようにします。
mod my;

fn function() {
  println!("called `function()`");
}

fn main() {
  my::function();

  function();

  my::indirect_access();

  my::nested::function();
  // my::nested::private_function(); // もちろんpub宣言がついていないのでアクセス付加

  // my/mod.rs で mod inaccesibleにpub宣言がついていないので
  // (inaccessible以下には)直接アクセス負荷
  // my::inaccessible::function();
  my::indirect_inaccessible(); //経由させると可能
}

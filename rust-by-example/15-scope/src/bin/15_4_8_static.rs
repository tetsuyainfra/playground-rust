// static lifetime
// - 'static lifetime はすべてのライフタイムの中で最長
//   - プログラムが動作している間有効
//   - 'static lifetimeでも圧縮されて短くなることはある
// - 'staticライフタイムを持つ変数を作る方法は2つ、いずれもバイナリの一部としてROMに格納される
//   - static変数とともに定数を作成
//   - 文字列リテラルで&'static str型を持つ変数を作成
//   ※ ROMってのは一般OSならバイナリのデータ領域てことかな？、組み込みがあるからこういう表現？

// static ライフタイムを持つ定数の作成
static NUM: i32 = 18;

// NUMへの参照を返す ライフタイムは'staticから引数のライフタイムに圧縮される
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
  &NUM
}

fn main() {
  {
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);
    // `static_string`がスコープから抜けると、参照は使用することが
    // できなくなるが、データはバイナリ中に残る。
  }
  {
    let lifetime_num = 9;
    let coerced_static = coerce_static(&lifetime_num);
    println!("coerced_static: {}", coerced_static);
  }
  println!("NUM: {} stays accessible!", NUM);

  {
    let ref parent_num: i32;
    let ref longlife_num: i32;
    {
      let life_base = 0;
      let ref nested_num: i32;
      // life_baseと同じライフタイムを持つので圧縮なし
      nested_num = coerce_static(&life_base);
      println!("nested_num: {}", nested_num);

      // life_baseと同じライフタイムでparent_numに参照が束縛
      parent_num = coerce_static(&life_base);
      println!("parent_num: {}", parent_num);

      // 比較にここで'staticのライフタイムを参照させる
      longlife_num = &NUM;

      // ここでlife_baseのライフタイムは終わる
      // = 圧縮されたparent_numのライフタイムも終わる
    }
    // 参照できずエラーになる
    // println!("parent_num: {}", parent_num); // Errorになる

    // こっちは問題ない
    println!("longlife_num: {}", longlife_num);
  }
}

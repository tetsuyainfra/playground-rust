// 圧縮 coerce
// 長いライフタイムを短く圧縮(coerce)することで
// そのままでは動作しないスコープの中でも使用できるようになる
// 以下の２パターンある
// Rustコンパイラが
//   - 推論の結果として圧縮する場合
//   - 複数のライフタイムを比較して圧縮する場合

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
  first * second
}

// これはライフタイム'aが最低でもライフタイム'bの長さと読める
// 'aの長さを受け取り'bの長さにして返す
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
  first
}

fn main() {
  let first = 2;
  {
    let second = 3;
    println!("The product is {}", multiply(&first, &second));
    // ここで帰り値のライフタイムは&second相当のライフタイムになってるんですかねー
    println!("{} is the first", choose_first(&first, &second));
  }
}

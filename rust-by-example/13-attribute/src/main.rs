// attributeはモジュール・クレート・要素に対するメタデータ
// 次がその使用目的
// - コンパイル時条件分岐
// - クレート名、バージョン、種類(バイナリ・ライブラリ)の設定
// - リント無効化
// - コンパイラ付属機能（マクロ、グロブ、インポート）の使用
// - 外部ライブラリへのリンク
// - ユニットテスト用関数を明示
// - ベンチマーク用関数を明示

// 書き方によって適用範囲が変わる
// #![crate_attribute] -> クレート全体に利用するとき
// #[item_attribute]   ->  モジュール、要素に適用するとき(!が無い)

// 引数の取り方
// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]

// 引数の取り方2
// 複数の値を取ることも
// #[attribute(value, value2)]

// 途中での改行も可能
// #[attribute(value, value2,
//                    value3, value4)]

fn main() {
    println!("Hello, world!");
}

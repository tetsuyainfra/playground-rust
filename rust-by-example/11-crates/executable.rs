// Link to `library`, import items under the `rary` module
// `library`にリンクし、`rary`モジュール内の要素を全てインポートする。
extern crate rary;

fn main() {
    rary::public_function();

    // エラー！`private_function`はプライベート
    //rary::private_function(); // --> Error

    rary::indirect_access();
}
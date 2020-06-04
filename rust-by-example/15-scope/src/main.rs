// RAII
// RAII(Resource Acquisition Is Initialization) リソース取得は初期化である, リソース確保は初期化時に等の意味
// Rustの変数はデータをスタックに保持するだけじゃなくBox<T>等を使ってヒープに確保したりする
// RustはRAIIを強制するので、オブジェクトがスコープを抜けると必ずデストラクタが呼び出され、
// オブジェクトが保持していた資源が解放される

// このふるまいはリソースバグを防ぐのに役立つ

fn create_box() {
    let _box1 = Box::new(3i32);

    // _box1はここで破棄される
}

fn main() {
    // 整数をヒープ上に確保
    let _box2 = Box::new(5i32);

    // ネストしたスコープ
    {
        // 整数をヒープ上に確保
        let _box3 = Box::new(4i32);

        // `_box3`はここで破棄され、メモリは解放される。
    }

    // お遊びで大量のボックスを作る。
    // もちろん手動で開放する必要はないよ！
    for _ in 0u32..1_000 {
        create_box();
    }

    droptest();
    // leak_test(); // memoryリークテスト

    // `_box2`はここで破棄され、メモリは解放される。
}

// デストラクタの概念はDropTraitで提供される
// このトレイトのみ全てのタイプに実装する必要がなく
// 独自のデストラクタロジックが必要な時にタイプを実装する
// 以下の例でDropトレイトの動作が確認できる

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn droptest() {
    let x = ToDrop;
    println!("Made a ToDrop");
    // ここでxがデストラクタされる
}

/*
Cargo-valgrind を使って下の代理をする
$ rustc raii.rs && valgrind ./raii

cargo install cargo-valgrind
cargo valgrind

cargo valgrind --bin hogehoge

エラーがあれば
Analyzing ``にメッセージが表示される
*/

use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    #[allow(dead_code)]
    fn puts(s: *const c_char);
}

#[allow(dead_code)]
fn leak_test() {
    let string = CString::new("Test").unwrap();

    let ptr = string.into_raw();
    unsafe { puts(ptr) };
}

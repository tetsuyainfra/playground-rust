// dead_code
// コンパイラはdead_codeと呼ばれるリント昨日を持つ
// 使用されていない関数が存在すると警告を出す
// アトリビュートで無効化できる

// 実際のコードで使用されていない箇所があれば"コードは除去するべき"

fn used_function() {}

// #[warn(dead_code)] <-- default
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)] // アンコメントするとワーニングは消える
fn noisy_unused_function() {}

fn main() {
  used_function();
}

// dynを使ってトレイトを返す

// Rustコンパイラは関数がすべての返す型がどれくらいサイズが必要か知る必要がある
// これはすべての関数は具象型(concrete type)であることを示す
// その他の言語と異なり、
// Animalトレイトを返す時、
// 実装が異なると異なる量のメモリが必要になるため
// Animalを返す関数を作成できません

// しかしながら、かんたんな回避方法があります
// トレイトオブジェクトを直接返す代わりに、関数でAnimalを”含む”Box型を返します。
// boxは単にヒープ上のメモリのどこかへの参照です
// その参照には静的で既知のサイズがあり、
// コンパイラはヒープに確保されたAnimalを指していることを保証できるため
// 関数からトレイトを返すことができる

// Rustは、ヒープにメモリを割り当てるときはいつでも、できるだけ明示的にしようとします。
// したがって、関数がこのような方法でヒープ上の特性へのポインターを返す場合、
// dynキーワードを使用して戻り値の型を記述する必要があります（例： Box<dyn Animal> 。

struct Sheep {}
struct Cow {}

trait Animal {
  fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaaaah!"
  }
}

impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "mooooooo!"
  }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

fn main() {
  let random_number = 0.234;
  let animal = random_animal(random_number);
  println!(
    "You've randomly choosen an animal, and it says {}",
    animal.noise()
  );
}

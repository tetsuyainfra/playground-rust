// Combinators: map
// match はOptionを扱うのに適したメソッドだが大量にこれを使うと面倒に感じる
// Some -> Some, あるいは None-> Noneの単純な操作を適用する場合
// map() とういうビルトインのメソッドがあるのでこれを利用する
// 複数のmap()はチェインできる

// 以下の例ではprocess()が直前の関数すべてを用いた場合と同じ機能をよりコンパクトに果たしている

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
  Apple,
  Carrot,
  Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// 食べ物の皮を剥く存在しない場合は単純にNoneを返す
// そうでなければ皮を剥いた食べ物のを返す
fn peel(food: Option<Food>) -> Option<Peeled> {
  match food {
    Some(food) => Some(Peeled(food)),
    None => None,
  }
}

// 上と同じように、食べ物を切る前に、皮を向いた食べ物の有無を知る必要がある。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
  match peeled {
    Some(Peeled(food)) => Some(Chopped(food)),
    None => None,
  }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
  chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
  food
    .map(|f| Peeled(f))
    .map(|Peeled(f)| Chopped(f))
    .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
  match food {
    Some(food) => println!("Mmm. I love {:?}", food),
    None => println!("Oh no it wasn't edible."),
  }
}

fn main() {
  let apple = Some(Food::Apple);
  let carrot = Some(Food::Carrot);
  let potato = None;

  let cooked_apple = cook(chop(peel(apple)));
  let cooked_carrot = cook(chop(peel(carrot)));
  let cooked_potato = process(potato);
  let cooked_potatos = process(Some(Food::Potato));

  eat(cooked_apple);
  eat(cooked_carrot);
  eat(cooked_potato);
  eat(cooked_potatos);
}

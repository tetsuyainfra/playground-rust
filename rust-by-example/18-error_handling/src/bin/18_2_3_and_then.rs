// Combinators: and_then

// map()はチェイン構文を用いてmatch文を単純化するもの
// Option<T>wo返す関数に対してのmap()の使用はネストしたOption<Option<T>>を生じさせる
// 複数の関数呼び出しをチェインさせることは混乱をまねきうる
// そこで and_then()の出番、多言語ではflatmapと呼ばれることもある

// and_then()は引数として与えられた関数にラップされた値を渡す
// その値がNoneだった場合はNoneを返す

// cookable_v2()はOption<Food>を返すため、and_then()ではなくmap()を使用すると
// 最終的にOption<Option<Food>>になる。これはeat()には不適切な型

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
  CordonBleu,
  Steak,
  Sushi,
}

#[derive(Debug)]
enum Day {
  Monday,
  Tuesday,
  Wednesday,
}

// 寿司の材料を持っていない
fn have_ingredients(food: Food) -> Option<Food> {
  match food {
    Food::Sushi => None,
    _ => Some(food),
  }
}

// コンドンブルーのレシピも持ってない
fn have_recipe(food: Food) -> Option<Food> {
  match food {
    Food::CordonBleu => None,
    _ => Some(food),
  }
}

// 料理を作るためには材料とレシピの両方が必要
// ロジックの流れをｍａｔｃｈおのチェインで表す
fn cookable_v1(food: Food) -> Option<Food> {
  match have_ingredients(food) {
    None => None,
    Some(food) => match have_ingredients(food) {
      None => None,
      Some(food) => Some(food),
    },
  }
}

fn cookable_v2(food: Food) -> Option<Food> {
  have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
  match cookable_v2(food) {
    Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
    None => println!("Oh no. We don't get to eat on {:?}?", day),
  }
}

fn main() {
  let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

  eat(cordon_bleu, Day::Monday);
  eat(steak, Day::Tuesday);
  eat(sushi, Day::Wednesday);
}

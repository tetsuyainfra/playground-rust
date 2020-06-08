// Trait
// トレイト(trait)とは任意の型となりうるSelfに対して定義されたメソッドの集合
// 同じトレイト内で宣言されたメソッド同士はお互いにアクセスすることができる
// トレイトはあらゆるデータ型に実装することができる

// 以下の例ではAnimalというメソッドの集合を定義し、
// その後AnimalトレイトをSheepというデータ型に対してして実装する
// これによりAnimalメソッドをSheepが使うことができる

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Staticメソッドのシグネチャ
    // Self型はこのトレイトを実装している方になる
    fn new(name: &'static str) -> Self;

    // インスタンスのシグネチャ
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // メソッドのデフォルト動作も定義できる
    fn talk(&self) {
        println!("{} says{}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    // Selfは実装対象の型、ここではSheep型
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // デフォルトのトレイトメソッドは上書きできる
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    //                     ^^^^^^ ここはTraitの名前であることに注意

    // 型アノテーションを消すと型がわからないので実態が作れない
    // let mut dolly = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

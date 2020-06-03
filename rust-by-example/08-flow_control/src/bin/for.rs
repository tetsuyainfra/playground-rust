fn main() {
  for_test();
  iter_test();
}

// for n in Iterator で 要素に対する処理ができる
// a..b で aからbの一つ前まで をyield(産出)できる
// a..=b で aからbまでという書き方ができる
fn for_test() {
  for n in 1..101 {
    if n % 15 == 0 {
      println!("fizzbuzz")
    } else if n % 3 == 0 {
      println!("fizz")
    } else if n % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", n)
    }
  }
  // --
  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz")
    } else if n % 3 == 0 {
      println!("fizz")
    } else if n % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", n)
    }
  }
}

// Iterator trait があって、 for loop では into_iter functionをコレクション(インスタンス)に適用する
// into_iter, iter, iter_mut はコレクションからイテレータにする異なる手段

fn iter_test() {
  let names = vec!["Bob", "Frank", "Ferris"];
  // iter() 各要素を借用する,コレクションはそのままでループ後再利用できる
  for name in names.iter() {
    match name {
      &"Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
  println!("Hello {}", names[0]);

  let names = vec!["Bob", "Frank", "Ferris"];
  // into_iter() 各要素を消費(所有権の移譲？)する,コレクションの要素はその後移動されるので再利用不可
  for name in names.into_iter() {
    match name {
      "Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
  // println!("Hello {}", names[0]); // moveされているのが検知される

  let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];
  // into_iter() 各要素を変更可能に借用する,コレクションの要素は変更可能に参照できる
  for name in names.iter_mut() {
    // match文の返り値で上書き
    *name = match name {
      &mut "Ferris" => "There is a rustacean among us!",
      _ => "Hello",
    }
  }
  println!("{:?}", names);
}

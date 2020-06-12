// ベクタ型が値を整数のインデックスで保持するのに対し、HashMapではキーで保持します。
// HashMapのキーはブーリアン、整数、文字列等のEqあるいはHashトレイトを保持する型なら何でもOKです。
// 後でより詳しく見ていきます。

// ベクタ型と同様、伸長可能ですが、
// HashMapの場合さらに、スペースが余っているときには小さくすることも可能です。
// HashMapを一定の容量のエリアに作成するときはHashMap::with_capacity(uint)を、
// デフォルトの容量で作成するときはHashMap::new()を用います。後者が推奨されています。

use std::collections::HashMap;

fn call(number: &str) -> &str {
  match number {
    "798-1364" => {
      "We're sorry, the call cannot be completed as dialed.
        Please hang up and try again."
    }
    "645-7689" => {
      "Hello, this is Mr. Awesome's Pizza. My name is Fred.
        What can I get for you today?"
    }
    _ => "Hi! Who is this again?",
  }
}

fn main() {
  let mut contacts = HashMap::new();

  contacts.insert("Daniel", "798-1364");
  contacts.insert("Ashley", "645-7689");
  contacts.insert("Katie", "435-8291");
  contacts.insert("Robert", "956-1745");

  match contacts.get(&"Daniel") {
    Some(&number) => println!("Calling Daniel: {}", call(number)),
    _ => println!("Don't have Daniel's number"),
  }

  contacts.remove(&"Ashley");

  // `HashMap::iter()`は(&'a key, &'a value)
  // のペアを順不同で産出するイテレータを返す
  for (contact, &number) in contacts.iter() {
    println!("Calling {}: {}", contact, call(number));
  }
}

// key型の変種
// EqとHashトレイトを実装している型ならば、なんでもHashMapのキーになることができます。例えば以下です。

// - bool （キーになりうる値が2つしかないので実用的ではないですが…）
// - int、uint、あるいは他の整数型
// - Stringと&str（Tips: Stringをキーにしたハッシュマップを作製した場合、.get()メソッドの引数に&strを与えて値を取得することができます。）
// - f32とf64はHashを実装して いない ことに注意しましょう。
// おそらくこれは浮動小数点演算時に誤差が発生するため、キーとして使用すると、恐ろしいほどエラーの元となるためです。

// 集合型は、その要素となっている全ての型がEqを、あるいはHashを実装している場合、必ず同じトレイトを実装しています。例えば、Vec<T>はTがHashを実装している場合、Hashを実装します。

// 独自の型にEqあるいはHashを実装するのは簡単です。以下の一行で済みます。 #[derive(PartialEq, Eq, Hash)]

// 後はコンパイラがよしなにしてくれます。これらのトレイトの詳細をコントロールしたい場合、EqやHashを自分で実装することもできます。この文書ではHashトレイトを実装する方法の詳細については触れません。

// structをHashMapで扱う際の例として、とてもシンプルなユーザーログインシステムを作成してみましょう。

use std::collections::HashMap;

// Eqトレイトを使用するときはPartialEqをderiveする必要がある
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
  username: &'a str,
  password: &'a str,
}

struct AccountInfo<'a> {
  name: &'a str,
  email: &'a str,
}

// キーと値に含まれる属性値の寿命が同じだと教えることで、
// 同時に利用できるようになってる
type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
  println!("Username: {}", username);
  println!("Password: {}", password);
  println!("Attempting logon...");

  let logon = Account { username, password };

  match accounts.get(&logon) {
    Some(account_info) => {
      println!("Successful logon!");
      println!("Name: {}", account_info.name);
      println!("Email: {}", account_info.email);
    }
    _ => println!("Login failed!"),
  }
}

fn main() {
  let mut accounts: Accounts = HashMap::new();

  let account = Account {
    username: "j.everyman",
    password: "password1",
  };
  let account_info = AccountInfo {
    name: "John EveryMan",
    email: "j.everyman@email.com",
  };

  accounts.insert(account, account_info);

  try_logon(&accounts, "j.everyman", "pass");
  try_logon(&accounts, "j.everyman", "password1");
}

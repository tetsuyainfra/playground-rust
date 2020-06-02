use std::convert::From;

fn main() {
    // 定義されてる
    let my_str = "hello";
    let my_string = String::from(my_str);

    // 定義してみる
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My Number is {:?}", num);

    // Type declaration が必要
    let num: Number = 5.into();
    println!("My Number is {:?}", num);
}

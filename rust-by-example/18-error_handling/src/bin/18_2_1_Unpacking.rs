// You can unpack Options by using match statements,
// あなたはオプションをマッチステートメントでアンパックできる
// but it's often easier to use the ? operator.
// しかし、あなたは ? operatorを使えばよりかんたんにできる
// If x is an Option,
// xがOptionのとき
// then evaluating x? will return the underlying value if x is Some,
// x? を評価すると
// otherwise it will terminate whatever function is being executed and return None.

fn next_birthday(current_age: Option<u8>) -> Option<String> {
  // If `current_age` is `None`, this returns `None`.
  // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
  let next_age: u8 = current_age?;
  Some(format!("Next year I will be {}", next_age))
}

struct Person {
  job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
  phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
  area_code: Option<u8>,
  number: u32,
}

impl Person {
  fn work_phone_area_code(&self) -> Option<u8> {
    // 本来ならmatchステートメントを使ってネストしないといけないが
    // ? をつかってそれを避けることができる
    self.job?.phone_number?.area_code
  }
}

fn main() {
  let p = Person {
    job: Some(Job {
      phone_number: Some(PhoneNumber {
        area_code: Some(61),
        number: 439222222,
      }),
    }),
  };

  assert_eq!(p.work_phone_area_code(), Some(61));

  // 追加
  let no_job_p = Person { job: None };
  assert_eq!(no_job_p.work_phone_area_code(), None);
}

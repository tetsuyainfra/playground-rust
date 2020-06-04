// New Type Idiom
// neytype idiomはコンパイル時にプログラムに正しい値を与えることを保証する
// 基本型を指定して新しい型を作るidiom

// 次の例は
// 年から年をチェックする関数に「必ずYears型の値を与えられる」ようになっている

struct Years(i64);

struct Days(i64);

impl Years {
  pub fn to_days(&self) -> Days {
    Days(self.0 * 365)
  }
}

impl Days {
  pub fn to_years(&self) -> Years {
    Years(self.0 / 365)
  }
}

fn old_enough(age: &Years) -> bool {
  age.0 >= 18
}

fn main() {
  let age = Years(5);
  let age_days = age.to_days();
  println!("Old enough {}", old_enough(&age));
  println!("Old enough {}", old_enough(&age_days.to_years()));
  // Yearsタイプである保証をさせているのでDays型で old_enoughを呼ぶとエラー
  // println!("Old enough {}", old_enough(&age_days)); // Errorになる

  // newtype の値を基本型として取得するには次のように書く
  let years = Years(42);
  let _years_as_primitive: i64 = years.0;
}

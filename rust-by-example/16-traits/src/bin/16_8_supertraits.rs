// Supertraits
// Rustは継承を持たない
// しかし他のトレイトをからのスーパーセット(上位集合)としてトレイトを定義することはできる

trait Person {
  fn name(&self) -> String;
}

// StudentはPersonの部分集合
trait Student: Person {
  fn university(&self) -> String;
}

trait Programmer {
  fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
  fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
  format!(
    "My name is {} and I attend {}. My Git username is {}",
    student.name(),
    student.university(),
    student.git_username()
  )
}

fn main() {}

// Iterating over Results
// An Iter::map operation might fail, for example:
fn main1() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
  println!("Results1: {:?}", numbers);
}
// Let's step through strategies for handling this.

// Ignore the failed items with filter_map()
// filter_map calls a function and filters out the results that are None.

fn main2() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Vec<_> = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .filter_map(Result::ok)
    .collect();
  println!("Results2: {:?}", numbers);
}

// Fail the entire operation with collect()
// Result implements FromIter so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>). Once an Result::Err is found, the iteration will terminate.
fn main3() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
  println!("Results3: {:?}", numbers);
}

// Collect all valid values and failures with partition()

fn main4() {
  let strings = vec!["tofu", "93", "18"];
  let (numbers, errors): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
  println!("Numbers4: {:?}", numbers);
  println!("Errors4: {:?}", errors);
}

// When you look at the results,
//  you'll note that everything is still wrapped in Result.
//  A little more boilerplate is needed for this.

fn main4_2() {
  let strings = vec!["tofu", "93", "18"];
  let (numbers, errors): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
  let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
  let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
  println!("Numbers42: {:?}", numbers);
  println!("Errors42: {:?}", errors);
}

fn main() {
  main1();
  main2();
  main3();
  main4();
  main4_2();
}

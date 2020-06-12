// Testcase: map-reduce
// Rust makes it very easy to parallelise data processing,
// without many of the headaches traditionally associated with such an attempt.
// Rustは並列データ処理を簡単にした。多くの伝統的なこれらの試行にまつわる頭痛を除いて

// The standard library provides great threading primitives out of the box.
// 標準ライブラリは、すぐに使える優れたスレッドプリミティブを提供します。
// These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.
//これらは、Rustの所有権の概念とエイリアスルールと組み合わせることで、データの競合を自動的に防止します。

// The aliasing rules (one writable reference XOR many readable references) automatically prevent you from manipulating state that is visible to other threads.
// エイリアシング規則（1つの書き込み可能な参照XORまたは多くの読み取り可能な参照）により、他のスレッドから見える状態を操作できなくなります。
//  (Where synchronisation is needed, there are synchronisation primitives like Mutexes or Channels.)
//  （同期が必要な場所には、ミューテックスやチャネルなどの同期プリミティブがあります。）

// In this example, we will calculate the sum of all digits in a block of numbers. We will do this by parcelling out chunks of the block into different threads.
// この例では、数値のブロックのすべての桁の合計を計算します。
//Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sums produced by each thread.
// これを行うには、ブロックのチャンクを別のスレッドにパーセル化します。各スレッドはその小さな数字のブロックを合計し、その後、各スレッドによって生成された中間合計を合計します。

// Note that, although we're passing references across thread boundaries,
// スレッドの境界を越えて参照を渡していますが、
//  Rust understands that we're only passing read-only references, and that thus no unsafety or data races can occur.
// Rustは、スレッドが終了するまでデータが存続していることを確認し、ぶら下がりポインターが発生しないようにします。
//  Because we're move-ing the data segments into the thread,
// データセグメントをスレッドに移動しているため、
//  Rust will also ensure the data is kept alive until the threads exit, so no dangling pointers occur.
// Rustは読み取り専用の参照しか渡していないことを理解しているため、安全性やデータの競合が発生しないことに注意してください。

use std::thread;

fn main() {
  // This is our data to process.
  // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
  // Each whitespace separated chunk will be handled in a different thread.
  //
  // TODO: see what happens to the output if you insert spaces!
  let data = "86967897737416471853297327050364959
  11861322575564723963297542624962850
  70856234701860851907960690014725639
  38397966707106094172783238747669219
  52380795257888236525459303330302837
  58495327135744041048897885734297812
  69920216438980873548808413720956532
  16278424637452589860345374828574668";
  println!("data: {}", &data);

  // Make a vector to hold the child-threads which we will spawn.
  let mut children = vec![];

  /*************************************************************************
   * "Map" phase
   *
   * Divide our data into segments, and apply initial processing
   ************************************************************************/

  // split our data into segments for individual calculation
  // each chunk will be a reference (&str) into the actual data
  let chunked_data = data.split_whitespace();

  // Iterate over the data segments.
  // .enumerate() adds the current loop index to whatever is iterated
  // the resulting tuple "(index, element)" is then immediately
  // "destructured" into two variables, "i" and "data_segment" with a
  // "destructuring assignment"
  for (i, data_segment) in chunked_data.enumerate() {
    println!("data segment {} is \"{}\"", i, data_segment);

    // Process each data segment in a separate thread
    //
    // spawn() returns a handle to the new thread,
    // which we MUST keep to access the returned value
    //
    // 'move || -> u32' is syntax for a closure that:
    // * takes no arguments ('||')
    // * takes ownership of its captured variables ('move') and
    // * returns an unsigned 32-bit integer ('-> u32')
    //
    // Rust is smart enough to infer the '-> u32' from
    // the closure itself so we could have left that out.
    //
    // TODO: try removing the 'move' and see what happens
    children.push(thread::spawn(move || -> u32 {
      // Calculate the intermediate sum of this segment:
      let result = data_segment
        // iterate over the characters of our segment..
        .chars()
        // .. convert text-characters to their number value..
        .map(|c| c.to_digit(10).expect("should be a digit"))
        // .. and sum the resulting iterator of numbers
        .sum();

      // println! locks stdout, so no text-interleaving occurs
      println!("processed segment {}, result={}", i, result);

      // "return" not needed, because Rust is an "expression language", the
      // last evaluated expression in each block is automatically its value.
      result
    }));
  }

  /*************************************************************************
   * "Reduce" phase
   *
   * Collect our intermediate results, and combine them into a final result
   ************************************************************************/

  // collect each thread's intermediate results into a new Vec
  let mut intermediate_sums = vec![];
  for child in children {
    // collect each child thread's return-value
    let intermediate_sum = child.join().unwrap();
    intermediate_sums.push(intermediate_sum);
  }

  // combine all intermediate sums into a single final sum.
  //
  // we use the "turbofish" ::<> to provide sum() with a type hint.
  //
  // TODO: try without the turbofish, by instead explicitly
  // specifying the type of final_result
  let final_result = intermediate_sums.iter().sum::<u32>();

  println!("Final sum result: {}", final_result);
}

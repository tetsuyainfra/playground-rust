///
/// SEE: https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html
/// - tracker.close()は追跡中のタスクをキャンセルできる
///   - close()後にspawnしたのは？ → 呼べちゃうのでやらないように
/// - tracker.wait().await で 終了を待つことができる
use tokio::time::{self, Duration};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

async fn background_task(num: u64) {
    for i in 0..10 {
        time::sleep(Duration::from_millis(100 * num)).await;
        println!("Background task {} in iteration {}.", num, i);
    }
}

#[tokio::main]
async fn main() {
    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    for i in 0..10 {
        let token = token.clone();
        tracker.spawn(async move {
            // Use a `tokio::select!` to kill the background task if the token is
            // cancelled.
            tokio::select! {
                () = background_task(i) => {
                    println!("Task {} exiting normally.", i);
                },
                () = token.cancelled() => {
                    // Do some cleanup before we really exit.
                    time::sleep(Duration::from_millis(50)).await;
                    println!("Task {} finished cleanup.", i);
                },
            }
        });
    }
    // let delayed_spawn = {
    //     // 遅延してspawnしてみる
    //     let tracker = tracker.clone();
    //     tokio::spawn(async move {
    //         println!("Delayed spawner start");
    //         println!("Delayed(0sec) task spawn");
    //         let _ = tracker.spawn(async move {
    //             println!("Delayed(0sec) task spawned");
    //         });

    //         time::sleep(Duration::from_secs(3)).await;

    //         println!("Delayed(1sec) task spawn");
    //         let _ = tracker.spawn(async move {
    //             tokio::task::yield_now().await;
    //             // task never spawned. because 3 secs afeter, tracker is closed.
    //             println!("Delayed(2sec) task spawned");
    //         });
    //         println!("Delayed spawner finish");
    //     })
    // };

    // Spawn a background task that will send the shutdown signal.
    {
        let tracker = tracker.clone();
        tokio::spawn(async move {
            // Normally you would use something like ctrl-c instead of
            // sleeping.
            time::sleep(Duration::from_secs(2)).await;
            // let _ = tokio::signal::ctrl_c().await;
            println!("Task will CLOSE");
            println!("background tracker.close(): {}", tracker.close());
            println!("Task CLOSE");

            // 
            // token.cancel();
        });
    }

    // Wait for delayed spawn
    // let _ = delayed_spawn.await;

    // Wait for all tasks to exit.
    tracker.wait().await;
    
    println!("main tracker.close(): {}", tracker.close());


    println!("All tasks have exited now.");
}

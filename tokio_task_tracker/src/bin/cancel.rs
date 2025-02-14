/// うーん使い方がよくわからない 早期終了できない・・・
/// 
/// When a call to wait returns,
///  it is guaranteed that all tracked tasks have exited and that the destructor of the future has finished running.
/// wait()の呼び出しが戻る時、追跡されているすべてのタスクが終了し、future のデストラクタの実行が完了したことが保証されます。
///  However, there might be a short amount of time where JoinHandle::is_finished returns false.
/// ただし、 JoinHandle::is_finishedからfalseが帰る時間が少しあります？
// タスクのデストラクタがきちんと呼ばれることが保障されてるってことなのかな？
// Cancelの追跡は自分でしないといけない？


use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() {
    let mut tracker = TaskTracker::new();

    for i in 0..5 {
        let t_tracker = tracker.clone();
        tracker.spawn(async move {
            println!("task{} 実行中...", i);
            println!("task{}: {}",i, t_tracker.is_closed() );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("task{}: {}",i, t_tracker.is_closed() );
            println!("タスク {} 完了!", i);
        });
    }
    
    println!("tracker.len(): {}", tracker.len());

    // 2秒後にキャンセル
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    tracker.close(); // すべてのタスクをキャンセル
    println!("tracker.len(): {}", tracker.len());
    tracker.wait().await;
    println!("tracker.len(): {}", tracker.len());

    println!("タスクはキャンセルされた!");
}
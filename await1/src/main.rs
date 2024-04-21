use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // 非同期タスクを生成して並行実行する
    let task1 = tokio::spawn(async_task("Task 1"));
    let task2 = tokio::spawn(async_task("Task 2"));
    let task3 = tokio::spawn(async_task("Task 3"));

    // 全てのタスクが完了するのを待つ
    let _task1 = task1.await.unwrap();
    let _task2 = task2.await.unwrap();
    let _task3 = task3.await.unwrap();
}

// 5秒間スリープする非同期関数
async fn async_task(name: &str) {
    println!("{} started", name);
    time::sleep(Duration::from_secs(5)).await;
    println!("{} finished", name);
}

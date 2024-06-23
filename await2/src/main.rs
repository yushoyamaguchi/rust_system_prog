use tokio::time::{Duration};
//use std::time::Duration;  // これだとエラーになる

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        wake_up().await;
    });

    //std::thread::sleep(Duration::from_secs(5));  //こっちに変えると、woke upのタスクが実行されないままプログラムが終わる
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Done");
}


async fn wake_up() {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("woke up!");
}

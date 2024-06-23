use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TCPリスナーを127.0.0.1:8080でバインド
    let listener = TcpListener::bind("127.0.0.1:10120").await?;

    println!("Server listening on 127.0.0.1:10120");

    loop {
        // 新しい接続を受け入れる
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];

    loop {
        // クライアントからデータを読み取る
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return, // 接続が終了
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        // データをエコー（返送）
        if let Err(e) = socket.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

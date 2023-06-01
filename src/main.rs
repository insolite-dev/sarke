/*
 * Just playing around here.
 *
 * The current code is an echo server that allows multiple connections.
 */

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut bufreader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = bufreader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}

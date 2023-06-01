/*
 * Just playing around here.
 *
 * Implements simple broadcast server that allows mutli-connection.
 */

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, adress) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        let message = format!("From: {} | {}", adress, line.clone());
                        tx.send((message, adress)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_adress) = result.unwrap();
                        if adress != other_adress {
                           writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

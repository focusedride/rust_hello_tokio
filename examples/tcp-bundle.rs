use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let e = tokio::spawn(echo());
    tokio::spawn(send());
    tokio::spawn(send());
    tokio::spawn(send());
    tokio::spawn(send());

    e.await.unwrap();
}

async fn send() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    let write_task = tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        wr.shutdown().await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}

async fn echo() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;
        // copy using io::copy
        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
        // copy manually
        // tokio::spawn(async move {
        //     let mut buf = vec![0; 1024];

        //     loop {
        //         match socket.read(&mut buf).await {
        //             Ok(0) => return,
        //             Ok(n) => {
        //                     println!("xxx {:?}", buf);
        //                 if socket.write_all(&buf[..n]).await.is_err() {
        //                     return;
        //                 }
        //             }
        //             Err(_) => {
        //                 return;
        //             }
        //         }
        //     }
        // });
    }
}

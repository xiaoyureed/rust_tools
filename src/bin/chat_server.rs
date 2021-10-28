use std::{
    io::{Error, ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc::{channel},
    time::Duration,
    vec,
};

/// server 在本地使用的端口
const LOCAL_ADDRESS: &str = "localhost:6000";
/// 传递消息的 buffer 大小
const MSG_BUF_SIZE: usize = 1024;

fn main() -> Result<(), Error> {
    // 连接目标地址
    // TcpStream::connect(LOCAL_ADDRESS);
    // 监听本地端口
    let listener = TcpListener::bind(LOCAL_ADDRESS)?;

    listener.set_nonblocking(true)?;

    // 暂存接收到的客户端
    let mut clients = vec![];
    let (s, r) = channel::<String>();
    loop {
        // 接收一个客户端的连接
        let (mut socket_client, addr_client) = listener.accept()?;
        //or
        // if let Ok((socket, addr_client)) = listener.accept() {
        //     println!("client {} connected", addr_client);
        // };
        println!("Client {} connected", addr_client);

        // copy 一份, 因为新建线程的时候使用了 move
        let s = s.clone();
        // copy 一份保存, 因为 socket 会被 move 到新建线程的内部
        clients.push(socket_client.try_clone()?);

        // 没接收一个 client , 新建一个线程处理, 这个线程循环读取 socket_client 中的数据
        std::thread::spawn(move || loop {
            let mut buf = vec![0; MSG_BUF_SIZE];

            // 读取消息
            match socket_client.read_exact(&mut buf) {
                Ok(_) => {
                    // 过滤出不为 0 的字节
                    let msg_from_buf = buf
                        .into_iter()
                        .take_while(|&ele| ele != 0)
                        .collect::<Vec<_>>();
                    // 转换为 string
                    let msg = String::from_utf8(msg_from_buf)
                        .expect("error of convert msg from u8 vec to string");
                    // 发送到 channel
                    s.send(msg).expect("error of send msg to sender");
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => {}
                Err(_) => {
                    println!("connection to {} closed.", addr_client);
                    break;
                }
            }

            // 让出 cpu
            sleep();
        });

        // println!("Write a msg:");

        // loop {
        //     let mut buf = String::new();
        //     stdin().read_line(&mut buf).expect("error of reading from stdin");
        //     let msg = buf.trim().to_string();
        //     if msg == ":quit" || s.send(msg).is_err() {
        //         break;
        //     }
        // }
    }

    // https://www.youtube.com/watch?v=CIhlfJSvxe4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&index=6
    // 非阻塞读取
    // match r.try_recv() {
    //     Ok(msg) => {
    //         let mut msg = msg.clone().into_bytes();
    //         // 修整 msg 长度, 超过 部分截断, 若不足部分填充 0
    //         msg.resize(MSG_BUF_SIZE, 0);
    //         socket_client
    //             .write_all(&msg)
    //             .expect("error of writing to socket");
    //         println!("send msg: {:?}", msg);
    //     }
    //     Err(TryRecvError::Disconnected) => break,
    //     Err(TryRecvError::Empty) => (),
    // }

    match r.try_recv() {
        Ok(msg) => {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut msg_bytes = msg.clone().into_bytes();
                    msg_bytes.resize(MSG_BUF_SIZE, 0);
                    client.write_all(&msg_bytes).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
        Err(_) => println!("error of try_recv"),
    }

    sleep();

    Ok(())
}

fn sleep() {
    std::thread::sleep(Duration::from_millis(100));
}

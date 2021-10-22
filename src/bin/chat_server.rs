use std::{io::{Error, ErrorKind, Read, Write}, net::TcpListener, sync::mpsc::channel, time::Duration, vec};

const LOCAL_ADDRESS: &str = "localhost:6000";
const MSG_BUF_SIZE: usize = 1024;

fn main() -> Result<(), Error> {
    // TcpStream::connect(LOCAL_ADDRESS);
    let listener = TcpListener::bind(LOCAL_ADDRESS)?;

    listener.set_nonblocking(true)?;

    let mut clients = vec![];
    let (s, r) = channel::<String>();
    loop {
        let (mut socket, addr_client) = listener.accept()?;
        //or
        // if let Ok((socket, addr_client)) = listener.accept() {
        //     println!("client {} connected", addr_client);
        // };

        let s = s.clone();
        clients.push(socket.try_clone()?);

        std::thread::spawn(move || loop {
            let mut buf = vec![0; MSG_BUF_SIZE];

            match socket.read_exact(&mut buf) {
                Ok(_) => {
                    let msg_from_buf = buf
                        .into_iter()
                        .take_while(|&ele| ele != 0)
                        .collect::<Vec<_>>();
                    let msg = String::from_utf8(msg_from_buf)
                        .expect("error of convert msg from u8 vec to string");
                    s.send(msg).expect("error of send msg to sender");
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => {}
                Err(_) => {
                    println!("connection to {} closed.", addr_client);
                    break;
                }
            }

            sleep();

        });
// todo https://www.youtube.com/watch?v=CIhlfJSvxe4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&index=4
        if let Ok(msg) = r.try_recv() {
            clients = clients.into_iter().filter_map(|client| {
                let mut buf = msg.clone().as_bytes();
                client.write_all(&mut buf).map(|_| client).ok();
            }).collect::<Vec<_>>();
        }

        sleep()
    }

    Ok(())
}

fn sleep() {
    std::thread::sleep(Duration::from_millis(100));
}

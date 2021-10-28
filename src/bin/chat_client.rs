use std::{
    io::{Error, ErrorKind, Read},
    net::TcpStream,
    sync::mpsc::channel,
    thread::spawn,
};

const SERVER_ADDR: &str = "localhost:6000";
const MSG_SIZE: usize = 32;

fn main() -> Result<(), Error> {
    let mut socket = TcpStream::connect(SERVER_ADDR)?;
    socket.set_nonblocking(true)?;

    let (s, r) = channel::<String>();

    spawn(move || loop {
        let mut buf = vec![0; MSG_SIZE];
        match socket.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf
                    .into_iter()
                    .take_while(|&it| it != 0)
                    .collect::<Vec<_>>();
                println!("recv msg: {:?}", &msg);
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => {}
            Err(_) => {
                println!("error occurred while socket.read_exact, break loop");
                break;
            }
        }

        match r.try_recv() {}
    });

    Ok(())
}

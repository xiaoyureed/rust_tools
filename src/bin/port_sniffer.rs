use std::{
    io::Write,
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread, vec,
};

/// 多线程扫描目标地址, 找出开放的端口
fn main() {
    // let args = std::env::args().collect::<Vec<String>>();
    //or
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let args = Args::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        }
        eprintln!("{}: err occurred while parse args -> {}", program, err);
        process::exit(0)
    });

    let (s, r) = channel::<u16>();

    let ip_addr = args.ip_addr;
    let threads = args.threads;
    for i in 0..args.threads {
        // 因为要放到新线程里, 所有权转移, 这里需要克隆一份
        let s = s.clone();
        thread::spawn(move || {
            scan(s, i, ip_addr, threads);
        });
    }

    let mut out = vec![];
    drop(s);
    for port in r {
        out.push(port);
    }
    out.sort();

    println!("");
    out.iter().for_each(|port| println!("{} is open", port));
}

const PORT_MAX: u16 = 65535;

fn scan(sender: Sender<u16>, port_start: u16, ip_addr: IpAddr, threads: u16) {
    let mut port = port_start + 1;
    loop {
        // 尝试连接目标地址
        match TcpStream::connect((ip_addr, port)) {
            // 嗅探成功
            Ok(_) => {
                // 打印一个点
                print!(".");
                std::io::stdout().flush().unwrap();
                // 将端口号发送到 channel
                sender.send(port).unwrap();
            }
            // 嗅探失败
            Err(_) => {}
        }

        // 若剩余的端口个数比线程个数还少, 结束嗅探循环
        if PORT_MAX - port <= threads {
            break;
        }

        // 因为有 多个 thread 同时嗅探, 所以循环每次会步进 thread 的个数
        port += threads;
    }
}

struct Args {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("args count not enough");
        }
        if args.len() > 4 {
            return Err("too many args");
        }
        //因为是 &[xxx], 所以要 clone
        let f = args[1].clone();
        if let Ok(ip) = IpAddr::from_str(&f) {
            return Ok(Args {
                flag: "".to_string(),
                ip_addr: ip,
                threads: 4,
            });
        }
        if f.contains("-h") || f.contains("--help") {
            if args.len() == 2 {
                println!(
                    "Usage: -j to select how many thread you want \r\n\r\n{}" ,"       -h or --help to show help message
                "
                );
                return Err("help message");
            }
            return Err("too many args");
        }
        if f.contains("-j") {
            let threads = match args[2].parse::<u16>() {
                Ok(n) => n,
                Err(_) => return Err("fail to parse thread count"),
            };
            let ip_addr = match IpAddr::from_str(&args[3]) {
                Ok(ip_addr) => ip_addr,
                Err(_) => return Err("not a valid ip addr"),
            };
            return Ok(Args {
                threads,
                ip_addr,
                flag: f,
            });
        }
        Err("invalid syntax")
    }
}

#[test]
fn t() {
    for i in 0..3 {
        println!("{}", i);
    }
}

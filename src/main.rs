use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn is_open(host: &String, port: u32) -> bool {
    TcpStream::connect(&*format!("{}:{}", host, port)).is_ok()
}

// fn opened_ports(host: &String, start: i32, finish: i32) -> Vec<i32> {
//     let mut open_ports: Vec<i32> = vec![];

//     for port in start..finish {
//         if !is_open(host, port) {
//             continue;
//         }
//         open_ports.push(port)
//     }

//     open_ports
// }

fn async_scan(host: &String, start: u32, end: u32) -> Vec<u32> {
    let mut open_ports: Vec<u32> = vec![];

    for port in start..end {
        if !is_open(host, port) {
            continue;
        }
        open_ports.push(port)
    }
    open_ports
}

fn main() {
    let mut host: String = "192.168.1.1".to_string();
    let mut ports: Vec<Vec<u32>> = Vec::new();
    let mut start_port: u32 = 1;
    let mut end_port: u32 = 65535; // max 65535
    let mut thread: u32 = 8;
    let mut count_thread: u32 = 0;
    let DEBUG = false;

    let step = end_port / thread;
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    for i in 0..thread {
            let host = host.clone();
            let tx = tx.clone();
            let tx2 = tx2.clone();

            thread::spawn(move || {
                if (DEBUG)
                {
                    println!("Thread[{}] : start : {} end : {}\n",i,&start_port,&step);
                }
                let vals = async_scan(&host, start_port, start_port + step);
                for elem in vals
                {
                    tx.send(elem).unwrap();
                }
                tx2.send(1).unwrap();
            });
            start_port += step;
        };
        
    let mut tot:u32 = 0;

    thread::spawn(move || {
        for received in rx {
            println!("Port : {}", received);
        }
    });

    for rec2 in rx2 {
        tot += rec2;
        if DEBUG 
        {
            println!("tot {}\n",&tot);
        }
        if tot == thread
        {   
            break;
        }
    }
}

/*
let conns = Arc::new( Mutex::new( vec![] ) );
let thread_list = (0..thread).into_iter().map(|i| {
        let host = host.clone();
        let mut conns = conns.clone();

        let handle = thread::spawn(move || {
            println!("Start : {} \nStep : {} \nEnd : {} \nTherad num : {}",start_port,step,start_port + step,thread);
            conns.lock().unwrap().push(async_scan(&host, start_port, start_port + step));
        });
        start_port += step;

        handle
    }).collect::<Vec<thread::JoinHandle<_>>>();

    for thr in thread_list {
        thr.join().unwrap();
    }
    let mut ports = conns.lock().unwrap();
    print_ports(&ports);
    */

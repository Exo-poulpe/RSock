use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::process;
use std::time::Duration;
use std::ascii;
use std::str;
use std::env;
use std::fs::File;
use std::io::Write;

use clap::{App,Arg};
use RSocklib;

// OPTIONS parser
fn options<'a>() -> clap::App<'a, 'a> {
    let result = App::new("RustHash")
                            .version("0.0.1.0")
                            .author("Exo-poulpe")
                            .about("Rust Socket scan network")
                            .arg(Arg::with_name("TARGET")
                                .short("t")
                                .long("target")
                                .required(false)
                                .takes_value(true)
                                .help("Set the IP to use"))
                            .arg(Arg::with_name("PORT")
                                .long("port")
                                .short("p")
                                .required(false)
                                .takes_value(true)
                                .help("Set the port to use"))
                            .arg(Arg::with_name("PORTSCAN")
                                .long("port-scan")
                                .required(false)
                                .takes_value(false)
                                .help("Make port scan on target"))
                            .arg(Arg::with_name("THREADS")
                                .long("thread")
                                .required(false)
                                .takes_value(true)
                                .help("Number of thread"))
                            .arg(Arg::with_name("VERBOSE")
                                .short("v")
                                .long("verbose")
                                .required(false)
                                .help("More verbose output (slower)"))                             
                            .arg(Arg::with_name("HELP")
                                .short("h")
                                .long("help")
                                .required(false)
                                .help("Print this message"));

    return result;
}

fn main() {
    let mut app : clap::App = options();
    let matches = app.clone().get_matches();

    let mut host : String;
    let mut start_port: u32 = 1;
    let end_port: u32 = 65535; // max 65535
    let mut thread: u32;
    let mut debug = false;
    
    if matches.is_present("PORTSCAN") && matches.is_present("TARGET") && !matches.is_present("HELP") {
        
        if matches.is_present("THREADS"){
            thread = matches.value_of("THREADS").expect("Fail to get value of target").parse::<u32>().expect("Fail to parse thread value");
        } else{
            thread = 4;
        }
        host = matches.value_of("TARGET").expect("Fail to get value of target").to_string();

        println!("Scan start at : {} number of threads : {}\n",&host,&thread);
        let result = RSocklib::port_discover(&host,&thread,&start_port,&end_port,&debug);
        
        for port in result {
            println!("Port : {}",port);
        }
        std::process::exit(0);
    }
    else
    {
        app.print_help();
        println!("\n");
    }
}
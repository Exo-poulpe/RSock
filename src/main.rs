#![crate_type = "bin"]
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::process;
use std::time::Duration;
use std::ascii;
use std::str;
use std::env;
use std::time::SystemTime;

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
                            .arg(Arg::with_name("CIDR")
                                .long("cidr")
                                .required(false)
                                .takes_value(true)
                                .help("Get CIDR for IP")) 
                            .arg(Arg::with_name("THREADS")
                                .long("thread")
                                .required(false)
                                .takes_value(true)
                                .help("Set number of thread"))
                            .arg(Arg::with_name("VERBOSE")
                                .short("v")
                                .long("verbose")
                                .required(false)
                                .help("More verbose output (slower)"))      
                            .arg(Arg::with_name("TIME")
                                .long("time")
                                .required(false)
                                .help("print time elapsed"))                       
                            .arg(Arg::with_name("HELP")
                                .short("h")
                                .long("help")
                                .required(false)
                                .help("Print this message"));

    return result;
}

fn main() {
    const default_thread_value : u32 = 8;
    const default_seconds_divisor : f64 = 1_000.;

    let mut app : clap::App = options();
    let matches = app.clone().get_matches();

    let mut host : String;
    let mut start_port: u32 = 1;
    let end_port: u32 = 65535; // max 65535
    let mut thread: u32;
    let mut verbose = matches.is_present("PORTSCAN");
    
    if matches.is_present("PORTSCAN") && matches.is_present("TARGET") && !matches.is_present("HELP") {
        
        if matches.is_present("THREADS"){
            thread = matches.value_of("THREADS").expect("Fail to get value of target").parse::<u32>().expect("Fail to parse thread value");
        } else{
            thread = default_thread_value;
        }

        host = matches.value_of("TARGET").expect("Fail to get value of target").to_string();
        let start = SystemTime::now();
        println!("Scan start for IP : {} number of threads : {}\n",&host,&thread);
        let result = RSocklib::port_discover(&host,&thread,&start_port,&end_port,&verbose);
        
        for port in result {
            println!("Port : {}",port);
        }

        let diff = start.elapsed().expect("Fail to get value of time").as_millis() as f64 / default_seconds_divisor as f64;
        if matches.is_present("TIME")
        {
            println!("\nTime elapsed : {} seconds",diff);
        }
        std::process::exit(0);
    } else if matches.is_present("PORT") && matches.is_present("TARGET") && !matches.is_present("HELP") {

        if matches.is_present("THREADS"){
            thread = matches.value_of("THREADS").expect("Fail to get value of target").parse::<u32>().expect("Fail to parse thread value");
        } else{
            thread = default_thread_value;
        }
        
        host = matches.value_of("TARGET").expect("Fail to get value of target").to_string();
        start_port = matches.value_of("PORT").expect("Fail to get value of target").parse::<u32>().expect("Fail to parse port");

        let start = SystemTime::now();

        println!("Scan start for IP : {}:{}\n",&host,&start_port);
        let result = RSocklib::is_open(&host,start_port);
        
        println!("The port : {} is {}",&start_port,if result { "open" } else { "close" });

        let diff = start.elapsed().expect("Fail to get value of time").as_millis() as f64;
        if matches.is_present("TIME")
        {
            println!("\nTime elapsed : {} seconds",diff);
        }
        std::process::exit(0);
    } else if matches.is_present("CIDR") && matches.is_present("TARGET") && !matches.is_present("HELP")
    {
        let host = matches.value_of("TARGET").expect("Fail to get value of target").to_string();
        let cidr = matches.value_of("CIDR").expect("Fail to get value of CIDR").parse::<u8>().unwrap();

        let mask = RSocklib::calc_cidr(&host,&cidr,&verbose);
        let wildcard = RSocklib::wildcard_mask(&cidr,&verbose);
        println!("Mask of network {}",mask);
        println!("Wild Mask of network {}",&wildcard);
        println!("Wildcard mask : {}",RSocklib::binary_ip_to_value(&wildcard));

        if matches.is_present("VERBOSE") {
            let start = mask.split(".").collect::<Vec<&str>>()[3].parse::<u8>().unwrap() + 1;
            let end = RSocklib::binary_ip_to_value(&wildcard).split(".").collect::<Vec<&str>>()[3].parse::<u8>().unwrap() - 1;
            println!("Port scan start at {} and stop at {}",start,end);
        }
    }
    else
    {
        app.print_help();
        println!("\n");
    }
}
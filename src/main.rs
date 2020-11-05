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
                                .help("Set hashes to test (file or string)"))
                            .arg(Arg::with_name("PORT")
                                .long("port")
                                .short("p")
                                .required(false)
                                .takes_value(true)
                                .help("Check if hash is valid"))
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

    let host = "192.168.1.1".to_string();
    let mut start_port: u32 = 1;
    let end_port: u32 = 65535; // max 65535
    let thread: u32 = 32;
    let debug = false;

    RSocklib::port_scanner(&host,&thread,&start_port,&end_port,&debug);

}
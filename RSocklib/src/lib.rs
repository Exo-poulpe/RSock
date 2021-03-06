use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::net::{TcpStream};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;
use std::time;
use std::collections::HashMap;



pub fn print_port_default(port:&u32) -> String {
    let default_port: HashMap<u32, &str> = 
    [(21,"ftp" ), (22, "ssh ?"),(23,"telnet ?"), (53,"dns ?"),(80, "http ?"),(115,"sftp ?"),(443,"https ?"),(989,"ftps ?"),(992,"telnet ?")]
    .iter().cloned().collect();
    let mut result;
    if default_port.contains_key(port) {
        result = format!("{} ({})",&port,default_port.get(port).unwrap());
    }else{
        result = port.to_string();
    }
    result
}

// Check if a port is open in a host return true or false
pub fn is_open(host: &String, port: u32) -> bool
{
    TcpStream::connect(&*format!("{}:{}", host, port)).is_ok()
}

// Async scan scan range of port on host and return all port open
pub fn async_scan(host: &String, start: u32, end: u32) -> Vec<u32>
{
    let mut open_ports = Vec::new();

    for port in start..end {
        if !is_open(host, port)
                {
                    continue;
                }
            open_ports.push(port)
        }
    open_ports
}

// Port scanner and send by channel list of port
fn port_scanner<'a>(
    host: &String,
    thread: &u32,
    start_port: &u32,
    end_port: &u32,
    tx : mpsc::Sender<Vec<u32>>,
    verbose: &bool,
){
    let step = end_port / thread;
    let verbose = *verbose;
    let mut tot: u32 = 0;
    let mut st_port = start_port.clone();

    for i in 0..*thread {
        let host = host.clone();
        let verbose = verbose.clone();
        let start_p = st_port.clone();
        let tx = tx.clone();
        thread::spawn(move|| {
            if verbose {
                println!("Thread[{}] : start : {} end : {}\n", i, &start_p, &step);
            }
            tx.send(async_scan(&host, start_p, start_p + step));
        });
        st_port += step;
    }
}

// Start port scan and listen the list returned by the port_scanner
pub fn port_discover<'a>(
    host: &String,
    thread: &u32,
    start_port: &u32,
    end_port: &u32,
    verbose: &bool,
) -> Vec<u32> {
    let (tx,rx) = mpsc::channel();
    let mut result : Vec<u32> = Vec::new();
    port_scanner(&host, &thread, &start_port, &end_port, tx, &verbose);
    for rec_v in rx {
        result.extend(rec_v);
    }
    result
}

// Calculate netmask from ip and CIDR
pub fn calc_cidr(ip : &String,cidr : &u8,verbose : &bool) -> String {
    let parsed_ip = &ip;

    let mut mask = create_mask(&cidr,&verbose);

    let v_ip = ip_to_vec(&ip);
    let v_mask = ip_to_vec(&mask);

    let mut v_id : Vec<u8> = Vec::new();

    for i in 0..v_ip.len()
    {
        v_id.push(v_ip[i]&v_mask[i]);
    }

    vec_to_ip(&v_id)

    
}

// Convert String ip to binary of ip
pub fn binary_ip_to_value(ip:&String) -> String {
    let v_ip = ip.split(".").collect::<Vec<&str>>();
    let mut result = String::new();

    for i in 0..v_ip.len() {
        result += &format!("{}",u8::from_str_radix(&v_ip[i].to_string(), 2).unwrap());
        if i != v_ip.len() - 1 {
            result += ".";
        }
    }
    result
}

// Calculate the wildcard mask
pub fn wildcard_mask(cidr:&u8,verbose : &bool) -> String{
    let mask = create_mask(&cidr,&verbose);
    let mut wild = String::new();
    let v_mask = ip_to_vec(&mask);
    let mut result = String::new();
    
    for i in v_mask {
        let tmp : &str = &format!("{:08b}",i);
        wild += tmp;
    }

    for (i, c) in wild.chars().enumerate() { 
        if i % 8 == 0 && i > 0 {
            result += ".";
        }
        if c == '0' {
            result += "1";
        }else{
            result += "0";
        }
    }
    result
}

// Convert vector to string ip
fn vec_to_ip(ip:&Vec<u8>) -> String 
{
    let mut s_ip : String = String::new();
    for i in 0..ip.len() {
        s_ip += &format!("{}",ip[i]);
        if i != ip.len() - 1{
            s_ip += ".";
        }
    }
    s_ip
}

// Convert ip to vector of u8
fn ip_to_vec(ip:&String) -> Vec<u8> {
    let mut result : Vec<u8> = Vec::new();
    let tmp = ip.split(".").collect::<Vec<&str>>();

    // println!("{:?}",tmp);
    for i in 0..4{
        result.push(tmp[i].parse::<u8>().unwrap());
    }
    result
}

// Create net mask from CIDR
pub fn create_mask(cidr : &u8,verbose : &bool) -> String {
    let mut tmp = String::new();
    const one : u32 = 1;
    const zero : u32 = 0;
    let mut v_mask : Vec<u8> = Vec::new();
    let mut result : String = String::new();

    for i in 0..32  {
        if i < *cidr {
            tmp += &one.to_string();
        }else
        {
            tmp += &zero.to_string();
        }
    }

    if *verbose 
    {
        println!("Mask tmp : {}",tmp);
    }

    for i in 0..4{
        v_mask.push(u8::from_str_radix(&tmp[i * 8..8 * (i + 1)], 2).unwrap());
    }

    if *verbose 
    {
        println!("{:?}",v_mask);
    }

    for i in 0..v_mask.len() {
        result += &v_mask[i].to_string();
        if i != v_mask.len() - 1 {
            result += ".";
        }

    }
    result
}

pub fn create_ip_range(ip:&String,mask:&String) -> (String,String) {

    let mut v_ip = ip_to_vec(&ip);
    let mut v_mask = ip_to_vec(&mask);
    let mut v_result = Vec::new();

    for i in 0..v_mask.len() {
        if v_mask[i] == 0 {
            v_result.push(v_ip[i]);
        }else{
            v_result.push(v_mask[i]);
        }
    }

    (vec_to_ip(&v_ip),vec_to_ip(&v_result))
    
}

pub fn scan_ip_range(start_ip : &String,end_ip : &String,verbose : &bool) -> Vec<String> {
    
    let mut vec_ip = ip_to_vec(start_ip);
    let mut result : Vec<String> = Vec::new();
    ping_addr(&vec_to_ip(&vec_ip),&end_ip,&verbose)
}

fn ping_addr(ip:&String,end_ip:&String,verbose:&bool) -> Vec<String>
{
    let (tx,rx) = mpsc::channel();
    let mut count : u32 = 0;
    let mut vec_ip = ip_to_vec(ip);
    let mut result : Vec<String> = Vec::new();

    loop {
        let tx = tx.clone();
        let v_ip = vec_ip.clone();
        let verbose = verbose.clone();
        thread::spawn(move || {
            ping(&vec_to_ip(&v_ip),tx,&verbose);
        });

        thread::sleep(time::Duration::from_millis(10));

        if vec_to_ip(&vec_ip) == *end_ip {
            break;
        }else{
            count += 1;
            if vec_ip[3] == 255 {
                vec_ip[3] = 1;
                if vec_ip[2] == 255 {
                    vec_ip[2] = 1;
                    vec_ip[1] += 1;
                }else{
                    vec_ip[2] += 1;
                }
            }else{
                vec_ip[3] += 1;
            }
        }
    }

    let mut l_cnt : u32 = 1;
    for rec_v in rx {
        if rec_v.1 == "true" {
            result.push(format!("{} : {}",rec_v.0,rec_v.1));
        }
        if l_cnt >= count {
            break;
        }else{
            l_cnt += 1;
        }
    }

    if *verbose {
        println!("Host scanned : {}",count);
    }
    
    result
}

fn ping(ip:&String,tx : mpsc::Sender<(String,&str)>,verbose:&bool) {
    let ip_to_ping = ip.parse::<IpAddr>().unwrap();
    let mut command = String::new();
    let mut result : bool;
    let mut args = Vec::new();

    if cfg!(unix) {
        args.push("-c");
    } else {
        args.push("-n");
    };
    args.push("5");
    args.push(ip);

    let mut cmd = Command::new("ping");
    cmd.args(&args[..]);
    result = cmd.output().expect("Error ping").status.success();
    // println!("ip {} : {}",ip,result);
    let mut sender = (ip.to_string(),if result {"true"} else {"false"});
    tx.send(sender);
}

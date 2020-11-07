use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::net::{TcpStream};

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
fn create_mask(cidr : &u8,verbose : &bool) -> String {
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

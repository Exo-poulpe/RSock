use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::process;
use std::time::Duration;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

// Check if a port is open in a host return true or false
pub fn is_open(host: &String, port: u32) -> bool {
    TcpStream::connect(&*format!("{}:{}", host, port)).is_ok()
}

// Async scan scan range of port on host and return all port open 
pub fn async_scan(host: &String, start: u32, end: u32) -> Vec<u32> {
    let mut open_ports: Vec<u32> = vec![];

    for port in start..end {
        if !is_open(host, port) {
            continue;
        }
        open_ports.push(port)
    }
    open_ports
}

// Save packet capture to file (TODO)
fn save_packet_file(filename: &str) {
    // Analyse packet return + detect service in packet
    let mut file = std::fs::File::create(filename).unwrap();

    let interface_name = "eth0";
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {

                let payload_offset;
                if interface.is_loopback() {
                    // The pnet code for BPF loopback adds a zero'd out Ethernet header
                    payload_offset = 14;
                } else {
                    // Maybe is TUN interface
                    payload_offset = 0;
                }
                if packet.len() > payload_offset {
                    let packet = EthernetPacket::new(packet).unwrap();

                    // file.write(packet);
                }
                // Constructs a single packet, the same length as the the one received,
                // using the provided closure. This allows the packet to be constructed
                // directly in the write buffer, without copying. If copying is not a
                // problem, you could also use send_to.
                //
                // The packet is sent once the closure has finished executing.
                // tx.build_and_send(1, packet.packet().len(),
                //     &mut |mut new_packet| {
                //         let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();

                //         // Create a clone of the original packet
                //         new_packet.clone_from(&packet);

                //         // Switch the source and destination
                //         new_packet.set_source(packet.get_destination());
                //         new_packet.set_destination(packet.get_source());
                // });
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }   
}


pub fn port_scanner<'a>(host:&String,thread:&u32,start_port:&u32,end_port:&u32,debug:&bool)
{
    
    // let h = &host.clone();
    // let thr = &thread.clone();
    // let mut s_port = start_port.clone();
    // let e_port= &end_port.clone();
    let step  = end_port / thread;
    let debug = *debug;
    // let h = host;
    // let thr = thread;
    // let mut s_port = start_port;
    // let e_port= end_port;
    // let step  = e_port  - s_port;
    // let debug = debug.clone();
    let connections = Mutex::new(Vec::new());
    let mut tot : u32 = 0;
    let (tx,rx) = mpsc::channel();
    let (tx2,rx2) = mpsc::channel();
    let mut st_port = start_port.clone();
    
    
    for i in 0..*thread {
        let host = host.clone();
        let tx = tx.clone();
        let tx2 = tx2.clone();

        thread::spawn(move || {
            if debug
            {
                println!("Thread[{}] : start : {} end : {}\n",i,&st_port,&step);
            }
            let vals = async_scan(&host, st_port, st_port + step);
            for elem in vals
            {
                tx.send(elem).unwrap();
            }
            tx2.send(1).unwrap();
        });
        st_port += step;
    };

    

    thread::spawn(move || {
        for received in rx {
            connections.lock().unwrap().push(received);
            if !debug {println!("Port : {}", received);}
        }
    });

    for rec2 in rx2 {
        tot += rec2;
        if debug 
        {
            println!("tot {}\n",&tot);
        }
        if tot == *thread
        {   
            break;
        }
    }

    
    
}
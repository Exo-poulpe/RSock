# *RSock* #

**RSock** is a program scan network and ip address.

## Installation ##

*This program has been tested only under linux*

Download the Rust source and compile then.

## Usage/Help ##

```
RSock 0.0.1.1
Exo-poulpe
Rust Socket scan network

USAGE:
    RSock [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Print this message
        --host-scan    Use target and cidr to scan host
        --port-scan    Make port scan on target
        --time         Print time elapsed
    -v, --verbose      More verbose output (slower)
    -V, --version      Prints version information

OPTIONS:
        --cidr <CIDR>         Get CIDR for IP
    -p, --port <PORT>         Set the port to use
    -t, --target <TARGET>     Set the IP to use
        --thread <THREADS>    Set number of thread
```

## Exemple scan ##

### Port ###
For example port scan:

```
./RSock -t 192.168.1.12 --port-scan --thread 16 --time
```

With this command you scan all port of the address 192.168.1.12 with 16 thread and print time elapsed for scan

Result :

```
Port : 80
Port : 667

Time elapsed : 18.45 seconds
```
(print only opened port)

### Host ###

For example host scan :

```
./RSock -t 192.168.1.1 --cidr 24 --host-scan --time
```
With this command you can scan all the network with CIDR number of net mask

Result : 
```
Mask of network 255.255.255.0
Network ID 192.168.1.0
Wildcard mask in binary of network 00000000.00000000.00000000.11111111 => 0.0.0.255
Port scan start at 1 and stop at 254
192.168.1.0 / 192.168.1.255
Host found : 192.168.1.1 : true
Host found : 192.168.1.112 : true
Host found : 192.168.1.45 : true

Time elapsed : 15.082 seconds
```

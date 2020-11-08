# *RSock* #

**RSock** is a program scan network and ip address.

## Installation ##

*This program has been tested only under linux*

Download the Rust source and compile then.

## Usage/Help ##

```
RSock 0.0.1.0
Exo-poulpe
Rust Socket scan network

USAGE:
    RSock [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Print this message
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

## Exemple ##

For exemple :

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

## Features ##

You can scan all network with ip and CIDR but this is single thread yet

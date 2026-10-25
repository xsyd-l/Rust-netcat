use std::env;
use std::net::TcpStream;
use std::io::{Read, Write};

fn is_all_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    for ch in s.chars() {
        if !ch.is_ascii_digit() {
            return false;
        }
    }
    true
}

fn is_valid_ipv4(s: &str) -> bool {
    let mut is_valid = true;
    let ip_numbers: Vec<&str> = s.split('.').collect();
    for number_str in ip_numbers {
        if !is_all_number(number_str) {
            is_valid = false;
        }
        if number_str.parse::<u32>().is_ok_and(|n| n > (1u32 << 8) - 1) {
            is_valid = false;
        }
    }
    is_valid
}

fn is_valid_port(s: &str) -> bool {
    let mut is_valid = true;
    if !is_all_number(s) {
        is_valid = false;
    } else if s.parse::<u32>().is_ok_and(|n| n > 65535u32 ) {
        is_valid = false;
    }
    is_valid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("error: netcat need args to work.\n try -h or --help to get more information.");
        return;
    }
    if args.len() == 2 {
        match args[1].as_str() {
            "-v" | "--version" => println!("Version information:\n    0.0.1\nNetwork layer protocol support:\n    Only IPv4 is supported.\nTransport layer protocol support:\n    Only TCP is supported.\nRole support:\n    The program only supports acting as a client."),
            "-h" | "--help" => println!("using netcat IPv4_addr port to connect or using\n-v  --version: get version information\n-h  --help: get more options"),
            _ => println!("error: invalid args\ntry -h or --help to get more information."),
        }
        return;
    }
    if args.len() != 3 {
        eprintln!("error: args number is invalid.")
    }
    let address = &args[1];
    let port = &args[2];
    if !is_valid_ipv4(address) {
        eprintln!("error: {} is not a valid IPv4 address.", address);
        return;
    }
    if !is_valid_port(port) {
        eprintln!("error: {} is not a valid port.", port);
        return;
    }
    let tcp_socket = String::from(address) + ":" + port;
    let mut stream = match TcpStream::connect(&tcp_socket) {
        Ok(s) => {
            println!("already connected {}", &tcp_socket);
            s
        },
        Err(e) => {
            eprintln!("connect failed: {}", e);
            return;
        }
    };
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    match stream.write_all(&mut line.as_bytes()) {
        Ok(value) => {},
        Err(e) => {
            eprintln!("errer: {}.", e);
        },
    }
}

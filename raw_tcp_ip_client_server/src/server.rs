use libc::*;
use std::ffi::CString;
use std::mem;
use tcp_server_client_syscall::{htonl, htons};

#[cfg(target_family = "unix")]
#[repr(C)]
pub struct SockAddrIn {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [i8; 8],
}

#[cfg(not(target_family = "unix"))]
#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [i8; 8],
}

fn main() {
    unsafe {
        // Step 1: Create a socket
        let fd = socket(AF_INET, SOCK_STREAM, 0);
        if fd < 0 {
            eprintln!("socket() failed");
            return;
        }

        // Step 2: Configure the socket (optional settings)
        let val: c_int = 1;
        if setsockopt(
            fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &val as *const _ as *const c_void,
            mem::size_of_val(&val) as u32,
        ) < 0
        {
            eprintln!("setsockopt() failed");
            close(fd);
            return;
        }

        // Step 3: Bind to an address
        let mut addr = SockAddrIn {
            #[cfg(target_family = "unix")]
            sin_len: mem::size_of::<SockAddrIn>() as u8,
            sin_family: AF_INET as u8,
            sin_port: htons(1234),
            sin_addr: in_addr {
                s_addr: htonl(INADDR_ANY),
            },
            sin_zero: [0; 8],
        };
        if bind(
            fd,
            &addr as *const _ as *const sockaddr,
            mem::size_of::<SockAddrIn>() as u32,
        ) < 0
        {
            eprintln!("bind() failed");
            close(fd);
            return;
        }

        // Step 4: Listen for connections
        if listen(fd, SOMAXCONN) < 0 {
            eprintln!("listen() failed");
            close(fd);
            return;
        }
        println!("Server listening on port 1234");

        // Step 5: Accept connections and handle them
        loop {
            let mut client_addr = SockAddrIn {
                #[cfg(target_family = "unix")]
                sin_len: 0,
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut addr_len = mem::size_of::<SockAddrIn>() as u32;
            let connfd = accept(
                fd,
                &mut client_addr as *mut _ as *mut sockaddr,
                &mut addr_len,
            );
            if connfd < 0 {
                eprintln!("accept() failed");
                continue;
            }
            println!("New client connected");

            // Handle the client in a new thread
            std::thread::spawn(move || {
                let mut buffer = [0; 64];
                let n = read(connfd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
                if n < 0 {
                    eprintln!("read() failed");
                    close(connfd);
                    return;
                }
                println!(
                    "Received: {}",
                    String::from_utf8_lossy(&buffer[..n as usize])
                );

                let response = CString::new("world").unwrap();
                write(
                    connfd,
                    response.as_ptr() as *const c_void,
                    response.as_bytes().len(),
                );
                close(connfd);
            });
        }
    }
}

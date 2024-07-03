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

        // Step 2: Connect to the server
        let addr = SockAddrIn {
            #[cfg(target_family = "unix")]
            sin_len: mem::size_of::<SockAddrIn>() as u8,
            sin_family: AF_INET as u8,
            sin_port: htons(1234),
            sin_addr: in_addr {
                s_addr: htonl(INADDR_LOOPBACK),
            },
            sin_zero: [0; 8],
        };
        if connect(
            fd,
            &addr as *const _ as *const sockaddr,
            mem::size_of::<SockAddrIn>() as u32,
        ) < 0
        {
            eprintln!("connect() failed");
            close(fd);
            return;
        }

        // Step 3: Send a message to the server
        let message = CString::new("hello").unwrap();
        write(
            fd,
            message.as_ptr() as *const c_void,
            message.as_bytes().len(),
        );

        // Step 4: Read the response from the server
        let mut buffer = [0; 64];
        let n = read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
        if n < 0 {
            eprintln!("read() failed");
            close(fd);
            return;
        }
        println!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..n as usize])
        );

        // Close the socket
        close(fd);
    }
}

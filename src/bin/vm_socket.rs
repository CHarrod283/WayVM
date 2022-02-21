use std::{env, thread};
use nix::sys::socket;
use std::path::Path;
use nix::sys::socket::{AddressFamily, SockAddr, SockFlag, SockType, UnixAddr};

mod pipe;


fn main() -> std::io::Result<()> {

    // create wayland "server"
    env::set_var("WAYLAND_DISPLAY", "wayland-1");
    let server_name = env::var("WAYLAND_DISPLAY").unwrap();
    let runtime_path = env::var("XDG_RUNTIME_DIR").unwrap();
    let server_path = format!("{}/{}", runtime_path, server_name);

    let server_addr = UnixAddr::new(Path::new(&server_path)).unwrap();
    let server_socket = socket::socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_CLOEXEC,
        None
    ).unwrap();

    socket::bind(server_socket, &SockAddr::Unix(server_addr)).unwrap();
    socket::listen(server_socket, pipe::MAX_CLIENTS).unwrap();

    loop {
        let client = socket::accept(server_socket).unwrap();
        let host_socket = socket::socket(
            AddressFamily::Vsock,
            SockType::Stream,
            SockFlag::SOCK_CLOEXEC,
            None
        ).unwrap();

        socket::connect(host_socket, &SockAddr::new_vsock(pipe::VMADDR_CID_HOST, pipe::PORT_NUM)).unwrap();
        thread::spawn(move || pipe::pipe(client, host_socket));
        thread::spawn(move || pipe::pipe(host_socket, client));
    }
}
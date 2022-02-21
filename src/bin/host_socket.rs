use std::{env, path, thread};
use std::path::Path;
use nix::NixPath;
use nix::sys::socket;
use nix::sys::socket::{AddressFamily, SockAddr, SockFlag, SockType};

mod pipe;

fn main() -> std::io::Result<()> {
    // Establish connection-oriented socket
    let vm_socket = socket::socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::SOCK_CLOEXEC,
        None
    ).unwrap();

    // Bind socket to local context id, port 5523
    socket::bind(vm_socket, &SockAddr::new_vsock(pipe::VMADDR_CID_ANY, pipe::PORT_NUM)).unwrap();

    socket::listen(vm_socket, pipe::MAX_CLIENTS).unwrap();

    let wl_name = env::var("WAYLAND_DISPLAY").unwrap();
    let runtime_path = env::var("XDG_RUNTIME_DIR").unwrap();
    let wl_path = format!("{}/{}", runtime_path, wl_name);
    let wl_addr = SockAddr::new_unix(Path::new(&wl_path)).unwrap();

    loop {
        let window_fd = socket::accept(vm_socket).unwrap();
        let wl_socket = socket::socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::SOCK_CLOEXEC,
            None
        ).unwrap();

        socket::connect(wl_socket, &wl_addr ).unwrap();
        thread::spawn(move || pipe::pipe(window_fd, wl_socket));
        thread::spawn(move || pipe::pipe(wl_socket, window_fd));
    }

}

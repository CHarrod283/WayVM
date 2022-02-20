use nix::sys::socket;

fn main() -> std::io::Result<()> {
    let cid = get_local_cid();

    // Establish connection-oriented socket
    let socket = socket::socket(
        socket::AddressFamily::Vsock,
        socket::SockType::Stream,
        socket::SockFlag::empty(),
        socket::SockProtocol::Tcp
    ).unwrap();

    // Bind socket to host context id, port 5523
    let socket_addr = socket::SockAddr::new_vsock(2, 5523);

    socket::bind(socket, &socket_addr).unwrap();

    socket::listen(socket, 32).unwrap();

    Ok(())
}
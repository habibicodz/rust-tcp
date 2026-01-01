use std::{
    io::{self, Read},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

pub fn run_tcp_server() -> Result<(), io::Error> {
    let port = 54321;
    let ip_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    let listener = TcpListener::bind(ip_addr).expect("Error starting listener");

    // Listenener
    println!("Listening for Tcp Clients");
    for conn in listener.incoming() {
        handle_tcp_stream(conn.unwrap());
    }

    Ok(())
}

fn handle_tcp_stream(mut stream: TcpStream) {
    let mut buff: [u8; 1500 + 4] = [0; 1500 + 4];
    stream.read(&mut buff).expect("Error reading data");
    println!("Data readed: {:?}", &buff);
}
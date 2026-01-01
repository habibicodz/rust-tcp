use std::{io::{self, Write}, net::{Ipv4Addr, SocketAddrV4, TcpStream}};

 pub fn run_tcp_client () -> Result<(), io::Error> {
    let port = 54321;
    let ip_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    let mut stream = TcpStream::connect(ip_addr).expect("Error connecting to server");
    stream.write(b"Hello world 000").expect("Error writing data to server");
    stream.flush()?;
    Ok(())
}
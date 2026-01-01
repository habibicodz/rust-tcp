mod sample;
mod tcp_client;
mod tcp_server;

use std::{
    io,
    thread::{self, sleep},
    time::Duration,
};

fn main() -> Result<(), io::Error> {
    let _ = thread::spawn(|| tcp_server::run_tcp_server());

    let client_thread_handle = thread::spawn(|| -> Result<(), io::Error> {
        sleep(Duration::from_secs(2));
        tcp_client::run_tcp_client()?;
        Ok(())
    });
    let _ = client_thread_handle.join().unwrap();

    sample::terminate_program();
    Ok(())
}

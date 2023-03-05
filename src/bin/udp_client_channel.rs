use std::sync::mpsc;
use std::{net::UdpSocket, thread};

fn main() {
    let addr = "127.0.0.1:5507";
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");

    let (tx, rx) = mpsc::sync_channel(10);

    thread::spawn(move || {
        let mut buf = [0u8; 4];
        for _ in 0..10 {
            let _ = socket.recv_from(&mut buf).expect("Didn't receive data");
            let t = f32::from_be_bytes(buf);
            tx.send(t).expect("couldn't send data");
            // thread::sleep(time::Duration::from_millis(1000));
        }
    });

    for t in rx {
        println!("Temperature: {}", t)
    }
}

use std::sync::{Arc, RwLock};
use std::{net::UdpSocket, thread, time};

struct Temperature {
    v: f32,
}

fn main() {
    let addr = "127.0.0.1:5507";
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");

    let temperature = Arc::new(RwLock::new(Temperature { v: 0.0 }));
    let temperarure_cln = temperature.clone();

    thread::spawn(move || {
        let mut buf = [0u8; 4];
        loop {
            let _ = socket.recv_from(&mut buf).unwrap();
            let t = f32::from_be_bytes(buf);
            let mut lock = temperarure_cln.write().expect("Didn't receive data");
            lock.v = t;
            drop(lock);
            // thread::sleep(time::Duration::from_millis(100));
        }
    });

    loop {
        let lock = temperature.read().unwrap();
        println!("Temperature: {:?}", lock.v);
        drop(lock);
        thread::sleep(time::Duration::from_millis(500));
    }
}

use std::net::UdpSocket;
use std::str::from_utf8;


pub fn udp_rx(ip_addr_port: String) {
    let socket = UdpSocket::bind(ip_addr_port).unwrap();

    let mut buf = [0; 256];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();

    let buf = &mut buf[..amt];
    println!("Address {}: \t {}", src, from_utf8(buf).unwrap());
}

pub fn udp_tx(ip_addr_port: String, dest_ip_addr_port: String, data: &[u8]) {
    let socket = UdpSocket::bind(ip_addr_port).unwrap();
    socket.send_to(data, dest_ip_addr_port).unwrap();
}
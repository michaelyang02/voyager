mod gamelib;

use std::thread;
use std::time::Duration;
use uuid::Uuid;
use gamelib::*;

fn main() {
    let t_handle = thread::spawn(|| {
        networking::udp_rx("127.0.0.1:24373".to_string());
    });
    thread::sleep(Duration::from_millis(1000));
    networking::udp_tx("127.0.0.1:24473".to_string(),
                       "127.0.0.1:24373".to_string(),
                       "Hello voyager!".as_bytes());
    t_handle.join().unwrap();
}
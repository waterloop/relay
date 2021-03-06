use std::sync::mpsc::{
    Sender,
    Receiver
};
use std::time::Duration;
use super::worker::UdpWorkerState;
use super::super::messages::*;
pub struct UdpManager {
}
use super::super::main_loop::WorkerStateTrait;

impl UdpManager {
    pub fn run<A: std::net::ToSocketAddrs+std::fmt::Debug+Send+'static>(
        can_sender: Sender<CanMessage>,
        tcp_sender: Sender<TcpMessage>,
        udp_receiver: Receiver<UDPMessage>,
        udp_max_number_timeouts: u32,
        udp_socket_read_timeout: Duration,
        udp_address: A
    ) -> std::thread::JoinHandle<()> {
        std::thread::Builder::new().name("UDP Thread".to_string()).spawn(move || {
            // Setup
            let mut udp_worker = UdpWorkerState::new(can_sender, tcp_sender, udp_receiver, udp_max_number_timeouts, udp_socket_read_timeout, udp_address);
            loop {
                udp_worker = udp_worker.main_loop();
            }
        }).expect("Should be able to create Thread")
    }
}

use super::packet::*;
use log::warn;
use serde::*;
use serde_derive::*;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::mpsc::Sender;
use std::thread;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    //socketの受信先
    pub multicast_ip4: [u8; 4],
    pub multicast_port: u16,
    //interface
    pub interface: [u8; 4],
    //buffer
    pub buffer_size: usize,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            multicast_ip4: [224, 5, 23, 2],
            multicast_port: 10030,
            interface: [0, 0, 0, 0],
            buffer_size: 4096,
        }
    }
}

#[derive(Debug)]
pub struct Radio {
    sender: Sender<Packet>,
    socket: UdpSocket,
}

impl Radio {
    pub fn spawn(settings: Settings, sender: Sender<Packet>) -> io::Result<()> {
        //変数定義
        let multicast_ip4 = Ipv4Addr::from(settings.multicast_ip4);
        let interface_ip4 = Ipv4Addr::from(settings.interface);
        let multicast_addr = SocketAddr::from((settings.multicast_ip4, settings.multicast_port));
        //ソケット作成
        let socket = UdpSocket::bind(multicast_addr)?;
        socket.join_multicast_v4(&multicast_ip4, &interface_ip4)?;
        //thread生成
        let radio = Radio {
            sender: sender,
            socket: socket,
        };
        thread::spawn(move || {
            let mut buffer = vec![0; settings.buffer_size];
            loop {
                if let Err(e) = radio.recv(&mut buffer) {
                    warn!("radio {:?}", e);
                }
            }
        });
        Ok(())
    }

    fn recv(&self, buffer: &mut [u8]) -> Result<(), String> {
        let size = self
            .socket
            .recv(buffer)
            .map_err(|e| format!("cannot receive {:?}", e))?;
        let packet = serde_json::from_slice(&buffer[0..size])
            .map_err(|e| format!("cannnot parse {:?}", e))?;
        self.sender
            .send(packet)
            .map_err(|e| format!("radio cannot enqueue {:?}", e))?;
        Ok(())
    }
}

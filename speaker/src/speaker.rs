use super::grSim_Commands::{grSim_Commands, grSim_Robot_Command};
use super::grSim_Packet::grSim_Packet;
use glm::Vec2;
use protobuf::Message;
use serde_derive::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::cell::RefCell;
use std::io;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub ip4: [u8; 4],
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            ip4: [224, 5, 23, 2],
            port: 20011,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Team {
    Yellow,
    Blue,
}

#[derive(Debug)]
pub struct Operation {
    //識別子
    pub team: Team,
    pub id: u32,
    //ロボットの移動
    pub speed: Vec2,
    pub rocation: f32,
    //キッカーについて
    pub kick: f32,
    pub chip: f32,
    pub spin: bool,
}

#[derive(Debug)]
pub struct Speaker {
    //ソケット
    buffer: RefCell<Vec<u8>>,
    socket: UdpSocket,
}

impl Speaker {
    pub fn new(settings: &Settings) -> io::Result<Speaker> {
        let addr = {
            let [a, b, c, d] = settings.ip4;
            Ipv4Addr::new(a, b, c, d)
        };
        let socket =(
            UdpSocket::bind(&SocketAddr::from((addr, settings.port))))?;
        socket
            .join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))?;
        Ok(Speaker {
            buffer: RefCell::new(Vec::new()),
            socket: socket,
        })
    }
    //注意:このメソッドはmulti-threadに対応していない。
    pub fn send(&self, op: &Operation) ->io::Result<()>{
        //データを変換する
        let mut command = grSim_Robot_Command::new();
        command.set_id(op.id);
        command.set_velangular(op.speed.x);
        command.set_velnormal(op.speed.y);
        command.set_veltangent(op.rocation);
        command.set_kickspeedx(op.kick);
        command.set_kickspeedz(op.chip);
        command.set_spinner(op.spin);
        let mut commands = grSim_Commands::new();
        commands.set_timestamp(0.0);
        commands.set_isteamyellow(op.team == Team::Yellow);
        commands.mut_robot_commands().push(command);
        let mut packet = grSim_Packet::new();
        packet.set_commands(commands);
        
        let mut buffer = self.buffer.borrow_mut();
        packet.write_to_vec(&mut buffer)?;
        self.socket.send(&buffer)?;
        Ok(())   
    }
}

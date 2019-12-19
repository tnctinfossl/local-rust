use super::grSim_Commands::{grSim_Commands, grSim_Robot_Command};
use super::grSim_Packet::grSim_Packet;

use glm::Vec2;
use protobuf::Message;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub ip4: [u8; 4],
    pub port: u16,
}

impl Settings {
    pub fn new(ip4: &[u8; 4], port: u16) -> Settings {
        Settings {
            ip4: ip4.clone(),
            port: port,
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            //ip4: [224, 5, 23, 2],
            ip4:[127,0,0,1],
            port: 20011,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Team {
    Yellow,
    Blue,
}

#[derive(Debug)]
pub struct OperationBuilder {
    //識別子
    pub team: Team,
    pub id: u32,
    //ロボットの移動
    pub speed: Vec2,
    pub rocation: f32,
    //キッカーについて
    pub kick_power: f32,
    pub chip_power: f32,
    pub spin: bool,
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
    pub kick_power: f32,
    pub chip_power: f32,
    pub spin: bool,
}

impl OperationBuilder {
    //要素数が大きいのでmethlod chainで追記してね
    pub fn new(team: Team, id: u32) -> OperationBuilder {
        OperationBuilder {
            team: team,
            id: id,
            speed: Vec2::new(0.0, 0.0),
            rocation: 0.0,
            kick_power: 0.0,
            chip_power: 0.0,
            spin: false,
        }
    }

    pub fn run(&mut self, speed: Vec2, rocation: f32) -> &mut OperationBuilder {
        self.speed = speed;
        self.rocation = rocation;
        self
    }

    pub fn kick(&mut self, power: f32) -> &mut OperationBuilder {
        self.kick_power = power;
        self
    }

    pub fn chip(&mut self, power: f32) -> &mut OperationBuilder {
        self.chip_power = power;
        self
    }

    pub fn spin(&mut self, sw: bool) -> &mut OperationBuilder {
        self.spin = sw;
        self
    }

    pub fn finalize(&self) -> Operation {
        Operation {
            team: self.team,
            id: self.id,
            speed: self.speed,
            rocation: self.rocation,
            kick_power: self.kick_power,
            chip_power: self.chip_power,
            spin: self.spin,
        }
    }
}

#[derive(Debug)]
pub struct Speaker {
    //ソケット
    socket: UdpSocket,
}

impl Speaker {
    pub fn new(settings: &Settings) -> io::Result<Speaker> {
        let socket = (UdpSocket::bind("localhost:0"))?;
        socket.connect(SocketAddr::from((settings.ip4,settings.port)))?;
        //socket.join_multicast_v4(&Ipv4Addr::from(settings.ip4),&Ipv4Addr::from([0,0,0,0]))?;
        Ok(Speaker {
            socket: socket
        })
    }
    //注意:このメソッドはmulti-threadに対応していない。
    pub fn send(&self, op: &Operation) -> io::Result<()> {
        let mut packet = grSim_Packet::new();
        packet.set_commands((|| {
            let mut commands = grSim_Commands::new();
            commands.set_timestamp(0.0);
            commands.set_isteamyellow(op.team == Team::Yellow);
            commands.set_robot_commands((|| {
                let mut items = Vec::new();
                items.push((|| {
                    let mut command = grSim_Robot_Command::new();
                    command.set_id(op.id);
                    command.set_veltangent(op.speed.x);
                    command.set_velnormal(op.speed.y);
                    command.set_velangular(op.rocation);
                    command.set_kickspeedx(op.kick_power);
                    command.set_kickspeedz(op.chip_power);
                    command.set_spinner(op.spin);
                    command.set_wheelsspeed(false);
                    command
                })());
                ::protobuf::RepeatedField::from(items)
            })());
            commands
        })());
        let mut buffer = Vec::new();
        packet.write_to_vec(&mut buffer)?;
        //println!("len={}:{:?}",buffer.len(),&buffer);
        self.socket.send(&buffer)?;
        Ok(())
    }
}

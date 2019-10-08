use super::grSim_Commands::{grSim_Commands, grSim_Robot_Command};
use super::grSim_Packet::grSim_Packet;
use super::grSim_Replacement::grSim_Replacement;
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
            ip4: [224, 5, 23, 2],
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
    buffer: RefCell<Vec<u8>>,
    socket: UdpSocket,
    settings: Settings,
}

impl Speaker {
    pub fn new(settings: &Settings) -> io::Result<Speaker> {
        let addr = {
            let [a, b, c, d] = settings.ip4;
            Ipv4Addr::new(a, b, c, d)
        };
        let socket = (UdpSocket::bind("localhost:0"))?;
        Ok(Speaker {
            buffer: RefCell::new(Vec::new()),
            socket: socket,
            settings: settings.clone(),
        })
    }
    //注意:このメソッドはmulti-threadに対応していない。
    pub fn send(&self, op: &Operation) -> io::Result<()> {
        //データを変換する
        /*
        let mut command = grSim_Robot_Command::new();
        let mut commands = grSim_Commands::new();
        commands.set_timestamp(0.0);
        commands.set_isteamyellow(op.team == Team::Yellow);
        commands.mut_robot_commands().push(command);
        let mut packet = grSim_Packet::new();
        packet.set_commands(commands);
        */
        let mut packet = grSim_Packet::new();
        //packet.mut_commands().set_timestamp(0.0);
        //socket.join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))?;
        packet.set_commands((|| {
            let mut commands = grSim_Commands::new();
            commands.set_timestamp(0.0);
            commands.set_isteamyellow(op.team == Team::Yellow);
            commands.set_robot_commands((|| {
                let mut items = Vec::new();
                items.push((|| {
                    let mut command = grSim_Robot_Command::new();
                    command.set_id(op.id);
                    command.set_velangular(op.speed.x);
                    command.set_velnormal(op.speed.y);
                    command.set_veltangent(op.rocation);
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
        
        //*packet.mut_replacement()=grSim_Replacement::new();
        //println!("fix {:?}", packet);
        //send target
        let mut buffer = self.buffer.borrow_mut();
        packet.write_to_vec(&mut buffer)?;

        let target = {
            let (ip4, port) = (self.settings.ip4, self.settings.port);
            format!("{}.{}.{}.{}:{}", ip4[0], ip4[1], ip4[2], ip4[3], port)
        };
        if let Err(e)=self.socket.send_to(&buffer, &target){
            println!("{:?}",e);
        }
        Ok(())
    }
}

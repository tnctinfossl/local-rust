use glm::Vec2;
use super::grSim_Commands::{grSim_Commands,grSim_Robot_Command};
use super::grSim_Packet::grSim_Packet;
use protobuf::Message;

#[derive(Debug,PartialEq)]
pub enum Team{
    Yellow,
    Blue
}

#[derive(Debug)]
pub struct Operation{
    //識別子
    pub team:Team,
    pub id:u32,
    //ロボットの移動
    pub speed:Vec2,
    pub rocation:f32,
    //キッカーについて
    pub kick:f32,
    pub chip:f32,
    pub spin:bool,
}

#[derive(Debug)]
pub struct Speaker{
    //ソケット
}

impl Speaker{
    pub fn new()->Speaker{
        Speaker{}
    }

    pub fn send(&self,op:&Operation){
        //データを変換する
        let mut command = grSim_Robot_Command::new();
        command.set_id(op.id);
        command.set_velangular(op.speed.x);
        command.set_velnormal(op.speed.y);
        command.set_veltangent(op.rocation);
        command.set_kickspeedx(op.kick);
        command.set_kickspeedz(op.chip);
        command.set_spinner(op.spin);
        let mut commands=grSim_Commands::new();
        commands.set_timestamp(0.0);
        commands.set_isteamyellow(op.team==Team::Yellow);
        commands.mut_robot_commands().push(command);
        let mut packet = grSim_Packet::new();
        packet.set_commands(commands);

       
        }
    
}
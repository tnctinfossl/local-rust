use crate::model::*;
use std::sync::mpsc::*;
use std::thread;
use serde_derive::*;
#[derive(Clone,Copy,Serialize,Deserialize)]
pub struct Settings{
    color:model::TeamColor,
    id:u32
}

impl Default for Settings{
    fn default()->Settings{
        Settings{
            color:TeamColor::Blue,
            id:1
        }
    }
}

pub struct AI{
    color:TeamColor,
    id:u32
}

impl AI {
    pub fn spawn(settings:&Settings,recv:Receiver<model::World>){
        let ai = AI{
            color:settings.color,
            id:settings.id
        };
        thread::spawn(move || {
            let mut world = model::World::new();
            loop{
                if let Ok(new_world)= recv.recv(){
                    world.merge(new_world,&model::MergeOptions::default());//TODO change default to arguments

                    ai.process(&world);
                }
            }
        });

    }

    fn process(&self,world:&model::World){
        println!("{:?}",world);
    }
}

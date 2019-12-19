extern crate speaker;
extern crate vision;
extern crate model;
extern crate ai;
use std::sync::mpsc::channel;
use vision::{Listener};
use std::collections::HashMap;
fn main(){
    //test code
    let settings=vision::Settings::default();

    //let world = Arc::new(RwLock::new(model::World::default()));
    let (tx,rx)=channel();
    Listener::spawn(&settings,tx).unwrap();
    ai::AI::spawn(&ai::Settings::default(),rx);
    //dummy
    let mut s=String::new();
    std::io::stdin().read_line(&mut s).ok();
    
}
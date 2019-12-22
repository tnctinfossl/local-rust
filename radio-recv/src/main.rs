extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
pub mod packet;
pub mod radio;
use packet::*;
use radio::*;
use std::sync::mpsc::channel;
fn main() {
    let settings = Settings::default();
    let (tx, rx) = channel();
    Radio::spawn(settings, tx).unwrap();
    while let Ok(p) = rx.recv() {
        println!("{:?}", p);
    }
}

use glm::*;
use serde_derive::*;
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum RobotID {
    Blue(u32),
    Yellow(u32),
}

impl Not for RobotID {
    type Output = Self;
    fn not(self) -> Self {
        use RobotID::*;
        match self {
            Blue(x) => Yellow(x),
            Yellow(x) => Blue(x),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfiled: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    //x,y,theta
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    //x,y
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

#[derive(Debug, Clone)]
pub struct Situation {
    pub balls: Vec<Ball>,
    pub robots: HashMap<RobotID, Robot>,
    pub field: Field,
}

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

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            position: vec3(0.0, 0.0, 0.0),
            velocity: vec3(0.0, 0.0, 0.0),
            acceleration: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl Robot {
    pub fn new(p: Vec3, v: Vec3, a: Vec3) -> Robot {
        Robot {
            position: p,
            velocity: v,
            acceleration: a,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    //x,y
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            position: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        }
    }
}

impl Ball {
    pub fn new(p: Vec2, v: Vec2, a: Vec2) -> Ball {
        Ball {
            position: p,
            velocity: v,
            acceleration: a,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Situation {
    pub balls: Vec<Ball>,
    pub robots: HashMap<RobotID, Robot>,
    pub field: Field,
}

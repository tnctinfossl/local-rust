use super::vec2rad::*;
use glm::*;
use rand::Rng;
use serde_derive::*;
use std::collections::HashMap;
use std::ops::Not;
// このファイルでは、mks単位系を採用する。

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

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    //x,y,theta
    pub position: Vec2Rad, //[m]
}

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            position: vec2rad(0.0, 0.0, 0.0),
        }
    }
}

impl Robot {
    pub fn new(p: Vec2Rad) -> Robot {
        Robot { position: p }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    //x,y
    pub position: Vec2,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            position: vec2(0.0, 0.0),
        }
    }
}

impl Ball {
    pub fn new(p: Vec2) -> Ball {
        Ball { position: p }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfiled: Vec2,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            infield: vec2(10.000, 10.000),
            outfiled: vec2(11.000, 11.000),
        }
    }
}

pub trait Contain<T> {
    fn contain(&self, rls: &T) -> bool;
}

impl Contain<Robot> for Field {
    fn contain(&self, rls: &Robot) -> bool {
        //領域 [begin,end\をもつ矩形とする
        let begin = -self.infield / 2.0;
        let end = self.infield / 2.0;
        //比較
        let check_x = begin.x < rls.position.x && rls.position.x < end.x;
        let check_y = begin.x < rls.position.y && rls.position.y < end.y;
        check_x && check_y
    }
}

impl Contain<Ball> for Field {
    fn contain(&self, rls: &Ball) -> bool {
        //領域 [begin,end\をもつ矩形とする
        let begin = -self.infield / 2.0;
        let end = self.infield / 2.0;
        //比較
        let check_x = begin.x < rls.position.x && rls.position.x < end.x;
        let check_y = begin.x < rls.position.y && rls.position.y < end.y;
        check_x && check_y
    }
}

pub trait RamdomReplace<T> {
    fn ramdom_replace<R: Rng + ?Sized>(&self, ramdom: &mut R) -> T;
}

impl RamdomReplace<Robot> for Field {
    fn ramdom_replace<R: Rng + ?Sized>(&self, ramdom: &mut R) -> Robot {
        use std::f32::consts::PI;
        Robot::new(vec2rad(
            ramdom.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
            ramdom.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
            ramdom.gen_range(0.0, 2.0 * PI),
        ))
    }
}

impl RamdomReplace<Ball> for Field {
    fn ramdom_replace<R: Rng + ?Sized>(&self, ramdom: &mut R) -> Ball {
        Ball::new(vec2(
            ramdom.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
            ramdom.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Situation {
    pub balls: Vec<Ball>,
    pub robots: HashMap<RobotID, Robot>,
    pub field: Field,
}

impl Situation {
    pub fn new(field: Field) -> Situation {
        Situation {
            balls: vec![],
            robots: HashMap::new(),
            field: field,
        }
    }
    #[allow(dead_code)]
    pub fn new_ramdom<R: Rng + ?Sized>(
        ramdom: &mut R,
        field: Field,
        blue: u32,
        yellow: u32,
        ball: u32,
    ) -> Situation {
        let robots = (0..blue)
            .map(|b| RobotID::Blue(b))
            .chain((0..yellow).map(|y| RobotID::Yellow(y)))
            .map(|id: RobotID| (id, field.ramdom_replace(ramdom)))
            .collect();
        let balls = (0..ball).map(|_| field.ramdom_replace(ramdom)).collect();
        Situation {
            balls: balls,
            robots: robots,
            field: field,
        }
    }
    #[allow(dead_code)]
    pub fn replace_ramdom<R: Rng + ?Sized>(&mut self, ramdom: &mut R) {
        let field = &self.field;
        for robot in self.robots.values_mut() {
            *robot = field.ramdom_replace(ramdom);
        }
        for ball in self.balls.iter_mut() {
            *ball = field.ramdom_replace(ramdom);
        }
    }
}

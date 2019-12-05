#[derive(Debug)]
struct Robot {
    x: f32,
    y: f32,
    angle: f32,
}

#[derive(Debug)]
struct Boll {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Space {
    x: f32,
    y: f32,
}


fn main() {
    let robot1 = Robot { x: -30.0, y: 30.0, angle: 0.0 };
    let robot2 = Robot { x: -15.0, y: 15.0, angle: -3.14 };
    let boll1 = Boll { x:0.0, y: 0.0 };

    let space = Space { x: robot1};//koko
    println!("robot1 is {:?}", robot1.x);
    println!("robot2 is {:?}", robot2);
}
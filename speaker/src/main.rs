use glm::Vec2;
use speaker::{Operation, OperationBuilder, Settings, Speaker, Team};
use std::f32::consts::PI;
fn main() {
    let settings = Settings::default();
    println!("{:?}", &settings);
    let speaker = Speaker::new(&settings).unwrap();
    println!("init speaker");
    let op = OperationBuilder::new(Team::Blue, 0)
        .run(Vec2::new(0.0, 0.0), PI * 2.0)
        .finalize();
    println!("send run");
    let mut number=0;
    loop {
        number+=1;
        println!("{}",number);
        speaker.send(&op).unwrap();
    }
}

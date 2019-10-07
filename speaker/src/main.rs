use speaker::{Operation, OperationBuilder, Settings, Speaker, Team};
use glm::Vec2;
fn main() {
    let settings = Settings::default();
    println!("{:?}",&settings);
    let speaker = Speaker::new(&settings).unwrap();
    println!("init speaker");
    let op = OperationBuilder::new(Team::Blue, 0).run(Vec2::new(0.0, 0.0), 0.1).finalize();
    speaker.send(&op).unwrap();
}

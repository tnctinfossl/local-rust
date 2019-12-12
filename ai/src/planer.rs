use super::objects::*;
use super::plot::*;
use super::vec2rad::*;
use glm::*;
use serde_derive::*;
//プラン作成時の制約条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanLmiter {
    pub filed: Field,
    pub velocity: f32,     //速度の上限[m/s]
    pub acceleration: f32, //加速度の上限[m/s^2]
}

impl Default for PlanLmiter {
    fn default() -> PlanLmiter {
        PlanLmiter {
            filed: Field::default(),
            velocity: 1.0,
            acceleration: 1.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gnuplot::*;
    use std::fs;
    fn demo_scene() -> Scene {
        let mut scene = Scene::new(Field::default());
        scene
            .robots
            .insert(Blue(0), Robot::new(vec2rad(1.0, 0.0, 0.0)));
        scene.balls.push(Ball::new(vec2(-1.0, 0.0)));
        scene
    }

    #[test]
    fn test_plan() {
        let mut figure = Figure::new();
        let mut scene = demo_scene();
        figure.plot(&scene);
        fs::create_dir_all("img");
        figure.save_to_png("img/test_plan.png", 1000, 1000);
    }
}

use super::objects::*;
use super::plot::*;
use super::vec2rad::*;
use glm::*;
use serde_derive::*;
//プラン作成時の制約条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub filed: Field,
    pub velocity: f32,     //速度の上限[m/s]
    pub acceleration: f32, //加速度の上限[m/s^2]
    pub avoid_radius: f32, //ロボットを避ける半径[m]
}

impl Default for Limit {
    fn default() -> Limit {
        Limit {
            filed: Field::default(),
            velocity: 1.0,
            acceleration: 1.0,
            avoid_radius: 0.3,
        }
    }
}

pub trait Plan {
    fn plan(&self, time: f32) -> Vec2Rad;
    //開始時間は0であるため、自明
    fn end_time(&self) -> f32; //終了時間
}

//障害物に構わず、直行する計画
pub struct DirectPlan {
    limit: Limit,
    begin: Vec2Rad,
    end: Vec2Rad,
}

impl DirectPlan {
    pub fn new(limit: Limit, begin: Vec2Rad, end: Vec2Rad) -> DirectPlan {
        DirectPlan {
            limit: limit,
            begin: begin,
            end: end,
        }
    }
}

impl Plan for DirectPlan {
    fn plan(&self, time: f32) -> Vec2Rad {
        self.begin + (self.end - self.begin) * (time / self.end_time())
    }
    fn end_time(&self) -> f32 {
        let begin2d = self.begin.to_vec2();
        let end2d = self.end.to_vec2();
        distance(begin2d, end2d) / self.limit.velocity
    }
}

//スプライン曲線を用いた計画
pub struct SplinePlan {
    limit: Limit,
    begin: Vec2Rad,
    end: Vec2Rad,
}

impl SplinePlan {}

#[cfg(test)]
mod test {
    use super::*;
    use gnuplot::*;
    use std::fs;

    const period: f32 = 1.0 / 60.0; //制御周期[s]

    fn demo_scene() -> Scene {
        let mut scene = Scene::new(Field::default());
        //自機
        scene
            .robots
            .insert(Blue(0), Robot::new(vec2rad(1.0, 0.0, 0.0)));
        //敵機
        scene
            .robots
            .insert(Yellow(0), Robot::new(vec2rad(0.0, 0.0, 0.0)));
        //目標地点
        scene.balls.push(Ball::new(vec2(-1.0, 0.0)));
        scene
    }

    #[test]
    fn test_direct_plan() {
        let mut figure = Figure::new();
        let mut scene = demo_scene();
        //計画の立案
        let limit = Limit::default();
        let begin = scene.robots[&Blue(0)].position;
        let end = Vec2Rad::from_vec2_rad(scene.balls[0].position, 0.0);
        let planer = DirectPlan::new(limit, begin, end);
        //計画の実行と記録
        let mut time = 0.0;
        let mut record = vec![];
        while time < planer.end_time() {
            record.push(planer.plan(time));
            time += period;
        }
        println!("{:?}", record);
        //描画
        let axes2d = figure.axes2d();
        axes2d.plot(&scene);
        let record_xs = record.iter().map(|p| p.x);
        let record_ys = record.iter().map(|p| p.y);
        axes2d.lines(
            record_xs,
            record_ys,
            &[
                PlotOption::LineStyle(DashType::Solid),
                PlotOption::LineWidth(1.0),
            ],
        );

        fs::create_dir_all("img");
        figure.show();
        figure.save_to_png("img/test_direct_plan.png", 1000, 1000);
    }
}

use super::objects::*;
use super::plot::*;
use super::vec2rad::*;
use glm::*;
use serde_derive::*;
//プラン作成時の制約条件
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Limit {
    pub filed: Field,
    pub period: f32,
    pub velocity: f32,     //速度の上限[m/s]
    pub acceleration: f32, //加速度の上限[m/s^2]
    pub avoid_radius: f32, //ロボットを避ける半径[m]
}

impl Default for Limit {
    fn default() -> Limit {
        Limit {
            period: 1.0 / 60.0,
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
        //
        let rate = self.limit.velocity * time / distance(self.begin.to_vec2(), self.end.to_vec2());
        //let rate = time * d /  / self.limit.period;
        self.begin + (self.end - self.begin) * rate
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

    #[test]
    fn test_direct_plan() {
        let mut figure = Figure::new();
        let begin = Vec2Rad::new(3.0, 0.0, 0.0);
        let end = Vec2Rad::new(-3.0, 0.0, 0.0);
        //計画の立案
        let limit = Limit::default();
        let mut scene = Scene::new(limit.filed);
        scene.robots.insert(RobotID::Blue(0), Robot::new(begin));
        scene.balls.push(Ball::new(end.to_vec2()));
        let plan = DirectPlan::new(limit, begin, end);
        //計画の実行と記録
        let mut time = 0.0;
        let mut record = Vec::new();
        while distance(plan.plan(time).to_vec2(), end.to_vec2()) > 0.01 {
            record.push(plan.plan(time));
            time += limit.period;
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

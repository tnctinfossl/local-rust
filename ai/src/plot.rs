extern crate gnuplot;
use super::objects::*;
use gnuplot::*;
pub trait Plotter<T> {
    fn plot(&mut self, targrt: &T);
}

impl Plotter<Scene> for Axes2D {
    fn plot(&mut self, scene: &Scene) {
        let size = 2.0;
        //blue
        let blues: Vec<_> = scene
            .robots
            .iter()
            .filter_map(|(k, v)| {
                if let RobotID::Blue(_) = k {
                    Some(v.position)
                } else {
                    None
                }
            })
            .collect();
        if blues.len() > 0 {
            let blue_xs = blues.iter().map(|p| p.x);
            let blue_ys = blues.iter().map(|p| p.y);
            self.points(
                blue_xs,
                blue_ys,
                &[
                    PlotOption::Color("blue"),
                    PlotOption::PointSize(size),
                    PlotOption::PointSymbol('O'),
                ],
            );
        }
        //yellow
        let yellows: Vec<_> = scene
            .robots
            .iter()
            .filter_map(|(k, v)| {
                if let RobotID::Yellow(_) = k {
                    Some(v.position)
                } else {
                    None
                }
            })
            .collect();
        if yellows.len() > 0 {
            let yellow_xs = yellows.iter().map(|p| p.x);
            let yellow_ys = yellows.iter().map(|p| p.y);
            self.points(
                yellow_xs,
                yellow_ys,
                &[
                    PlotOption::Color("orange"),
                    PlotOption::PointSize(size),
                    PlotOption::PointSymbol('O'),
                ],
            );
        }
        //ball
        if scene.balls.len() > 0 {
            let ball_xs = scene.balls.iter().map(|b| b.position.x);
            let ball_ys = scene.balls.iter().map(|b| b.position.y);
            self.points(
                ball_xs,
                ball_ys,
                &[
                    PlotOption::Color("red"),
                    PlotOption::PointSize(size),
                    PlotOption::PointSymbol('O'),
                ],
            );
        }
        //枠
        let infield = scene.field.infield;
        let rect_xs = [
            -infield.x / 2.0,
            infield.x / 2.0,
            infield.x / 2.0,
            -infield.x / 2.0,
            -infield.x / 2.0,
        ];
        let rect_ys = [
            -infield.y / 2.0,
            -infield.y / 2.0,
            infield.y / 2.0,
            infield.y / 2.0,
            -infield.y / 2.0,
        ];
        self.lines(
            rect_xs.iter(),
            rect_ys.iter(),
            &[PlotOption::Color("black"), PlotOption::PointSize(size)],
        );
    }
}

impl Plotter<Robot> for Axes2D {
    fn plot(&mut self, robot: &Robot) {
        let size = 2.0;
        self.points(
            [robot.position.x].iter(),
            [robot.position.y].iter(),
            &[
                PlotOption::Color("gray"),
                PlotOption::PointSize(size),
                PlotOption::PointSymbol('o'),
            ],
        );
    }
}
#[cfg(test)]
mod test {
    use super::super::vec2rad::*;
    use super::*;
    use std::fs;
    #[test]
    fn test_plot_scene() {
        let mut figure = Figure::new();
        let mut ramdom = rand::thread_rng();
        let field = Field::default();
        let scene = Scene::new_ramdom(&mut ramdom, field, 10, 10, 1);
        figure.axes2d().plot(&scene);
        fs::create_dir_all("img").unwrap();
        figure
            .save_to_png("img/test_plot_scene.png", 1000, 1000)
            .unwrap();
    }
    #[test]
    fn test_plot_robot() {
        let mut figure = Figure::new();
        let robot = Robot::new(vec2rad(0.0, 0.0, 0.0));
        figure.axes2d().plot(&robot);
        fs::create_dir_all("img").unwrap();
        figure
            .save_to_png("img/test_plot_robot.png", 1000, 1000)
            .unwrap();
    }
}

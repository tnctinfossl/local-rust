extern crate gnuplot;
use super::objects::*;
use gnuplot::*;
pub trait Plotter<T> {
    fn plot(&mut self, targrt: &T);
}

impl Plotter<Situation> for Figure {
    fn plot(&mut self, situation: &Situation) {
        let axes2d = self.axes2d();
        //blue
        let blues: Vec<_> = situation
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
        let blue_xs = blues.iter().map(|p| p.x);
        let blue_ys = blues.iter().map(|p| p.y);
        axes2d.points(blue_xs, blue_ys, &[PlotOption::Color("blue")]);
        //yellow
        let yellows: Vec<_> = situation
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

        let yellow_xs = yellows.iter().map(|p| p.x);
        let yellow_ys = yellows.iter().map(|p| p.y);
        axes2d.points(yellow_xs, yellow_ys, &[PlotOption::Color("orange")]);
        //ball
        let ball_xs = situation.balls.iter().map(|b| b.position.x);
        let ball_ys = situation.balls.iter().map(|b| b.position.y);
        axes2d.points(ball_xs, ball_ys, &[PlotOption::Color("red")]);
        //枠
        let infield = situation.field.infield;
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
        axes2d.lines(
            rect_xs.iter(),
            rect_ys.iter(),
            &[PlotOption::Color("black")],
        );
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;
    #[test]
    fn test_plot() {
        let mut figure = Figure::new();
        let mut ramdom = rand::thread_rng();
        let field = Field::default();
        let situation = Situation::new_ramdom(&mut ramdom, field, 10, 10, 1);
        figure.plot(&situation);
        fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();
    }
}

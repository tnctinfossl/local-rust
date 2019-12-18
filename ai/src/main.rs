//use glm::{vec2,Vec2};
use glm::distance;
use gnuplot::*;
//use std::cmp::Ordering;
//use std::io;
use glm::*;

/*
//二点a,bを通る線分と点pの距離
#[allow(dead_code)]
pub fn distance_segment_point((a, b): (Vec2, Vec2), p: Vec2) -> f32 {
    let tt = -dot(b - a, a - p) / length2(a - b);
    if tt < 0.0 {
        return distance(p, a);
    }
    if tt > 1.0 {
        return distance(p, b);
    }
    return distance_line_point((a, b), p);
}
*/
fn dodge_point(point1:f32,point2:f32,aa:Vec2,bb:Vec2) -> Vec2{
    if point1 < point2{
        println!("回避ポイント1");
        return aa;
    }else{
        println!("回避ポイント2");
        return bb;
    }
}

fn distance_min_point(dis_up:f32,dis_down:f32,dis_right:f32,dis_left:f32) -> f32{
  
    if dis_up < dis_down {
        if dis_up < dis_right{
            if dis_up < dis_left{
                println!("1番小さいのはup");
                return dis_up;
            }else{
                println!("1番小さいのはleft");
                return dis_left;
            }
        }else{
            if dis_right < dis_left{
                println!("1番小さいのはright");
                return dis_right;
            }else{
                println!("1番小さいのはleft");
                return dis_left;
            }
        }
    }else{
        if dis_down < dis_right{
            if dis_down < dis_left{
                println!("1番小さいのはdown");
                return dis_down;
            }else{
                println!("1番小さいのはright");
                return dis_right;
            }
        }else{
            if dis_right < dis_left{
                println!("1番小さいのはright");
                return dis_right;
            }else{
                println!("1番小さいのはleft");
                return dis_left;
            }
        }
    }
}

fn distance_min2_point(dis_up:f32,dis_down:f32,dis_right:f32,dis_left:f32) -> f32{
  
    if dis_up < dis_down {
        if dis_up < dis_right{
            if dis_up < dis_left{
                if dis_down < dis_right{
                    if dis_down < dis_left{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }else{
                    if dis_right < dis_left{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }
            }else{
                if dis_up < dis_down{
                    if dis_up < dis_right{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }
                }else{
                    if dis_down < dis_right{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }
                }
            }
        }else{
            if dis_right < dis_left{
                if dis_up < dis_down{
                    if dis_up < dis_left{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }else{
                    if dis_down < dis_left{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }
            }else{
                if dis_up < dis_down{
                    if dis_up < dis_right{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }
                }else{
                    if dis_down < dis_right{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }
                }
            }
        }
    }else{
        if dis_down < dis_right{
            if dis_down < dis_left{
                if dis_up < dis_right{
                    if dis_up < dis_left{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }else{
                    if dis_right < dis_left{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }
            }else{
                if dis_up < dis_down{
                    if dis_up < dis_left{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }else{
                    if dis_down < dis_left{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }
            }
        }else{
            if dis_right < dis_left{
                if dis_up < dis_down{
                    if dis_up < dis_left{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }else{
                    if dis_down < dis_left{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはleft");
                        return dis_left;
                    }
                }
            }else{
                if dis_up < dis_down{
                    if dis_up < dis_right{
                        println!("2番目に小さいのはup");
                        return dis_up;
                    }else{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }
                }else{
                    if dis_down < dis_right{
                        println!("2番目に小さいのはdown");
                        return dis_down;
                    }else{
                        println!("2番目に小さいのはright");
                        return dis_right;
                    }
                }
            }
        }
    }
}

fn main() {
    let robot1 =vec2(-15.0,20.0); 
    let robot2 =vec2(-5.0,15.0);
    let ball =vec2(0.0,0.0);

    let mut figure = gnuplot::Figure::new();
    let axes2d: &mut Axes2D = figure.axes2d();
    let robot_xs = [robot1.x,robot2.x];
    let robot_ys = [robot1.y,robot2.y];
    axes2d.points(&robot_xs,&robot_ys,&[PlotOption::Color("red"),PlotOption::PointSize(5.0)]);
 
    let ball_xs = [ball.x];
    let ball_ys = [ball.y];
    axes2d.points(&ball_xs,&ball_ys,&[PlotOption::Color("blue"),PlotOption::PointSize(5.0)]);
    
    let r = 5.0;
    let points_xs = [robot2.x,robot2.x,robot2.x+r,robot2.x-r];
    let points_ys = [robot2.y+r,robot2.y-r,robot2.y,robot2.y];
    axes2d.points(&points_xs,&points_ys,&[PlotOption::Color("black"),PlotOption::PointSize(5.0)]);

    let points_up = vec2(robot2.x,robot2.y+r);
    let points_down = vec2(robot2.x,robot2.y-r);
    let points_right = vec2(robot2.x+r,robot2.y);
    let points_left = vec2(robot2.x-r,robot2.y);
    let dis_up = distance(robot1,points_up);
    let dis_down = distance(robot1,points_down);
    let dis_right = distance(robot1,points_right);
    let dis_left = distance(robot1,points_left);
/*
    let dis_up = 5.0;
    let dis_down = 4.0;
    let dis_right = 3.0;
    let dis_left = 2.0;*/


    let a = distance_min_point(dis_up, dis_down, dis_right, dis_left);
    let b = distance_min2_point(dis_up, dis_down, dis_right, dis_left);

    let mut dis_ball1 = 0.0;
    let mut dis_ball2 = 0.0;
    let mut aa = vec2(0.0,0.0);
    let mut bb = vec2(0.0,0.0);

    if a == dis_up{
        dis_ball1 = distance(ball,points_up);
        aa = points_up;
        if b == dis_down{
            dis_ball2 = distance(ball,points_down);
            bb = points_down;
        }
        else if b == dis_right{
            dis_ball2 = distance(ball,points_right);
            bb = points_right;
        }
        else if b == dis_left{
            dis_ball2 = distance(ball,points_left);
            bb = points_left;
        }
    }
    else if a == dis_down{
        dis_ball1 = distance(ball,points_down);
        aa = points_down;
        if b == dis_up{
            dis_ball2 = distance(ball,points_up);
            bb = points_up;
        }
        else if b == dis_right{
            dis_ball2 = distance(ball,points_right);
            bb = points_right;
        }
        else if b == dis_left{
            dis_ball2 = distance(ball,points_left);
            bb = points_left;
        }
    }
    else if a == dis_right{
        dis_ball1 = distance(ball,points_right);
        aa = points_right;
        if b == dis_up{
            dis_ball2 = distance(ball,points_up);
            bb = points_up;
        }
        else if b == dis_down{
            dis_ball2 = distance(ball,points_down);
            bb = points_down;
        }
        else if b == dis_left{
            dis_ball2 = distance(ball,points_left);
            bb = points_left;
        }
    }
    else if a == dis_left{
        dis_ball1 = distance(ball,points_left);
        aa = points_left;
        if b == dis_up{
            dis_ball2 = distance(ball,points_up);
            bb = points_up;
        }
        else if b == dis_right{
            dis_ball2 = distance(ball,points_right);
            bb = points_right;
        }
        else if b == dis_down{
            dis_ball2 = distance(ball,points_down);
            bb = points_down;
        }
    }else{
        println!("aaa");
    }

    
    let c = dodge_point(dis_ball1, dis_ball2,aa,bb);
    
    println!("{:?}",c);
    let dodge_xs = [c.x];
    let dodge_ys = [c.y];
    axes2d.points(&dodge_xs,&dodge_ys,&[PlotOption::Color("green"),PlotOption::PointSize(5.0)]);
 
    axes2d.set_x_range(Fix(-40.0), Fix(10.0));
    axes2d.set_y_range(Fix(-10.0), Fix(40.0));
    figure.show();

    
    /*
    let r = 5.0;
    let ofsets = [vec2(0.0,r),vec2(0.0,-r),vec2(r,0.0),vec2(-r,0.0)];
    let mut points = Vec::new();
 
    println!("{:?}",robot1);

    for q in ofsets.iter(){
        points.push(*q+robot2);
    }
    println!("{:?}",points);
*/
}



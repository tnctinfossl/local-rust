//use glm::{vec2,Vec2};
use glm::distance;
use gnuplot::*;
//use std::cmp::Ordering;
//use std::io;
use std::time::{Duration, Instant};
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
/*
fn spline(robot1:f32,c:f32,ball:f32,t:f32) -> f32{

    let M = mat3(0.0,(1.0/60*(1.0/60.0),4.0*(1.0/60.0)*(1.0/60.0),
                0.0,1.0/60.0,1.0/30.0,
                1.0,1.0,1.0);
    let matM = inverse(&M);  //逆行列

    let mat_x = vec3( 
        robot1,c,ball
    );
    
    let mat_xx = matM*mat_x;
    let mat_xa = mat_xx.x;
    let mat_xb = mat_xx.y;
    let mat_xc = mat_xx.z;
    let matx = mat_xa*tx*tx+mat_xb*tx+mat_xc;
    return matx;
}
*/
fn spline(xs:&[f32;3],delta_time:f32)->Box <dyn Fn(f32)->f32>{
    let d=delta_time;
    //Mx=~y
    let m=mat3(0.0,d*d,4.0*d*d,0.0,d,2.0*d,1.0,1.0,1.0);
    let y=vec3(xs[0],xs[1],xs[2]);
    let x=m.inverse().unwrap()*y; 
    Box::new(move|t:f32|{
        let tt = vec3(t*t,t,1.0);
        dot(tt,x)
    })
}


fn spline_point(ps:&[Vec2;3],delta_time:f32)->Box <dyn Fn(f32)->Vec2>{
    let d=delta_time;
    let xs = [ps[0].x,ps[1].x,ps[2].x];
    let ys = [ps[0].y,ps[1].y,ps[2].y];
    let fx =spline(&xs,d);
    let fy =spline(&ys,d);
    Box::new(move|t:f32|{
        vec2(fx(t),fx(d))
    })
}

/*
fn next_pointy(robot1:f32,c:f32,ball:f32,ty:f32) -> f32{
    
    let M = mat3(0.0,(1.0/60.0)*(1.0/60.0),4.0*(1.0/60.0)*(1.0/60.0),
    0.0,1.0/60.0,1.0/30.0,
    1.0,1.0,1.0);
    let matM = inverse(&M);  //逆行列

    let mat_y = vec3( 
    robot1,c,ball
    );

    let mat_yy = matM*mat_y;
    let mat_ya = mat_yy.x;
    let mat_yb = mat_yy.y;
    let mat_yc = mat_yy.z;
    let maty = mat_ya*ty*ty+mat_yb*ty+mat_yc;
    return maty;
}
*/
fn dodge_point(point1:f32,point2:f32,aa:Vec2,bb:Vec2) -> Vec2{ 
    //2つの候補点のうちボール(目的地)に近い方を求める関数
    if point1 < point2{
        println!("回避ポイント1");
        return aa;
    }else{
        println!("回避ポイント2");
        return bb;
    }
}

fn distance_min_point(dis_up:f32,dis_down:f32,dis_right:f32,dis_left:f32) -> f32{
  //4つの候補点のうち移動するロボットから一番近い点を求める関数
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
  //4つの候補点のうち二番目に近い点を求める関数
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
    let start = Instant::now(); //時間計測開始
    let robot1 =vec2(-6.0,30.0); //移動するロボットの座標
    let robot2 =vec2(-5.0,15.0); //避ける対象のロボットの座標
    let ball =vec2(-10.0,5.0); //ボール(目的地)の座標

    //移動するロボットと避ける対象のロボットの描画
    let mut figure = gnuplot::Figure::new();
    let axes2d: &mut Axes2D = figure.axes2d();
    let robot_xs = [robot1.x,robot2.x];
    let robot_ys = [robot1.y,robot2.y];
    axes2d.points(&robot_xs,&robot_ys,&[PlotOption::Color("red"),PlotOption::PointSize(5.0)]);
 
    //ボール(目的地)の描画
    let ball_xs = [ball.x];
    let ball_ys = [ball.y];
    axes2d.points(&ball_xs,&ball_ys,&[PlotOption::Color("blue"),PlotOption::PointSize(5.0)]);
    
    //4つの候補点の描画
    let r = 5.0;
    let points_xs = [robot2.x,robot2.x,robot2.x+r,robot2.x-r];
    let points_ys = [robot2.y+r,robot2.y-r,robot2.y,robot2.y];
    axes2d.points(&points_xs,&points_ys,&[PlotOption::Color("black"),PlotOption::PointSize(5.0)]);

    //移動するロボットから4つの候補点までの距離
    let points_up = vec2(robot2.x,robot2.y+r);
    let points_down = vec2(robot2.x,robot2.y-r);
    let points_right = vec2(robot2.x+r,robot2.y);
    let points_left = vec2(robot2.x-r,robot2.y);
    let dis_up = distance(robot1,points_up);
    let dis_down = distance(robot1,points_down);
    let dis_right = distance(robot1,points_right);
    let dis_left = distance(robot1,points_left);


    let a = distance_min_point(dis_up, dis_down, dis_right, dis_left);
    let b = distance_min2_point(dis_up, dis_down, dis_right, dis_left);

    let mut dis_ball1 = 0.0;
    let mut dis_ball2 = 0.0;
    let mut aa = vec2(0.0,0.0);
    let mut bb = vec2(0.0,0.0);

    //1番近い候補点と2番目に候補点の代入
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

    //候補点の座標の確定
    let c = dodge_point(dis_ball1, dis_ball2,aa,bb);
    
    //候補点の描画
    println!("{:?}",c);
    let dodge_xs = [c.x];
    let dodge_ys = [c.y];
    axes2d.points(&dodge_xs,&dodge_ys,&[PlotOption::Color("green"),PlotOption::PointSize(5.0)]);



    let delta = 1.0/60.0;
    for x1 in 0..21 {
        let y1 = x1 as f32;
        let sfx =spline(&[robot1.x,c.x,ball.x],delta);
        let x =sfx(0.1*y1*delta);
        let sfy = spline(&[robot1.y,c.y,ball.y],delta);
        let y = sfy(0.1*y1*delta);
        let xx = [x];
        let yy = [y];
    
    
    
    println!("{}",x);
    println!("{}",y);
    axes2d.points(&xx,&yy,&[PlotOption::Color("red"),PlotOption::PointSize(1.0)]);
    }
    //println!("{:?}",mx);
    //println!("{:?}",my);
    
    let end = start.elapsed(); //時間計測終了

    //グラフの縦軸と横軸の設定
    axes2d.set_x_range(Fix(-40.0), Fix(10.0));
    axes2d.set_y_range(Fix(-10.0), Fix(40.0));
    
    figure.show(); //グラフ描画
    
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);

}



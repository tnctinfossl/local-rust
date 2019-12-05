use glm::vec2;


/*#[derive(Debug)]
struct Space {
    upx: f32,
    upy: f32,
    downx: f32,
    downy: f32,
    rightx: f32,
    righty: f32,
    leftx: f32,
    lefty: f32,
}
*/


fn main() {
    /*let robot1 = vec2(-30.0,30.0);
    let robot2 = vec2(-15.0,15.0);
    let boll1 = vec2(0.0,0.0);

    let space = vec2(robot2.x,robot2.y+5.0,robot2.x,robot2.y-5.0);
    let space = Space { upx: robot2.x, upy: robot2.y+5.0,
                        downx: robot2.x, downy: robot2.y-5.0,
                        rightx: robot2.x+5.0, righty: robot2.y,
                        leftx: robot2.x-5.0, lefty: robot2.y, };
                    

    println!("robot1 is {:?}", robot1);
    println!("robot2 is {:?}", space);
    */
  
    let p =vec2(-15.0,15.0);
    let ofsets = [vec2(0.0,5.0),vec2(0.0,-5.0),vec2(5.0,0.0),vec2(-5.0,0.0)];
    let mut points = Vec::new();
 
    
    for q in ofsets.iter(){
        
        points.push(*q+p);
        println!("{:?}",points);
    }
}
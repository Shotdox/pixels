use super::Vec2;

fn compute_next(current: &mut Vec2, constant: &Vec2){
    let new_x = current.x * current.x - current.y * current.y + constant.x;
    current.y = 2. * current.x * current.y + constant.y;
    current.x = new_x;
}


pub fn mod2(z: &mut Vec2) -> f64{
    return z.x * z.x + z.y * z.y;
}


pub fn compute_iterations(z0: &mut Vec2, constant: &Vec2, max_iterations: usize) -> usize{
    
    let mut iteration: usize = 0;
    while iteration < max_iterations && mod2(z0) < 1024.{
        //println!("{:?}", z0);
        compute_next(z0, constant);
        iteration += 1;
    }
    return iteration;
}
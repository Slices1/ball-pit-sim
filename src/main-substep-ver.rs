#![feature(get_many_mut)]
use macroquad::prelude::*;
use ::rand::Rng;

#[derive(Clone)]
struct Ball {
    radius: f32,
    position: Vec2,
    velocity: Vec2,
}

fn spawn_ball(position: Vec2, balls: &mut Vec<Ball>) {
    
    balls.push(Ball {
        radius: rand::gen_range(5., 15.),
        position: position,
        velocity: Vec2{x:0.,y:0.},
    });
}

fn check_collides(ball_1: &Ball, ball_2: &Ball) -> bool {
    //if centre distance is less than sum of radii
    if (ball_1.position.x - ball_2.position.x).powi(2) + (ball_1.position.y - ball_2.position.y).powi(2) < (ball_1.radius + ball_2.radius).powi(2) {
        return true;
    }
    return false;
}

fn normalise(my_vector: Vec2) -> Vec2 {
    return my_vector/(my_vector.x.powi(2) + my_vector.y.powi(2)).sqrt();
}

fn ball_to_ball_collision(ball_1: &mut Ball, ball_2: &mut Ball, c: f32) {
    let u1 = ball_1.velocity;
    let u2 = ball_2.velocity;
    let mass1 = (ball_1.radius).powi(2);
    let mass2 = (ball_2.radius).powi(2);
    let line_of_centres: Vec2 = normalise(ball_2.position - ball_1.position);
    
    

    let u1_perp: Vec2;
    let u1_paral: Vec2;
    let u2_perp: Vec2;
    let u2_paral: Vec2;

    //find mapped u1 and u2
    u1_paral = u1.dot(line_of_centres)*line_of_centres;
    u1_perp = u1 - u1_paral;
    u2_paral = u2.dot(line_of_centres)*line_of_centres;
    u2_perp = u2 - u2_paral;

    


    //compute new parallel component
    let new_u1_paral = (u1_paral*(mass1 -c*mass2)+mass2*u2_paral*(c+1.))/(mass1 + mass2);
    let new_u2_paral = (u2_paral*(mass2 -c*mass1)+mass1*u1_paral*(c+1.))/(mass1 + mass2);

    

    //set the new velocities
    ball_1.velocity = new_u1_paral + u1_perp;
    ball_2.velocity = new_u2_paral + u2_perp;

    

    
    //move them apart
    let magnitude: f32 = ((ball_2.position.x - ball_1.position.x).powi(2) + (ball_2.position.y - ball_1.position.y).powi(2)).sqrt() - ball_1.radius - ball_2.radius;
    
    ball_1.position += line_of_centres*magnitude/2.;
    ball_2.position += -line_of_centres*magnitude/2.;


}

#[macroquad::main("Ball Pit Sim in Rust")]
async fn main() {
    set_fullscreen(true);
    for _i in 1..=5 { // so full screen has time to open
        clear_background(GRAY);
        next_frame().await;
    }

    let mut balls: Vec<Ball> = Vec::new();
    
    let gravity: f32 = 0.08;
    let ball_to_wall_c: f32 = 0.90;
    let ball_to_ball_c: f32 = 0.80;

    for i in 1..=60 {
        for j in 1..=35 {
            spawn_ball(Vec2{x:30. + 30.*(i as f32) + 2.*(j as f32),y:30. + 30.*(j as f32)}, &mut balls);
        }
    }
    
    loop {
        clear_background(GRAY);
        //spawn
        

        //update positions
        for ball in &mut balls {
            ball.velocity.y += gravity;
            ball.position += ball.velocity;
        }


        //resolve ball-to-the-wall collisions

        for ball in &mut balls {
            // left and right wall
            if ball.position.x < ball.radius {
                ball.position.x = ball.radius;
                ball.velocity.x *= -ball_to_wall_c;
                
            } else if ball.position.x > screen_width()-ball.radius {
                ball.position.x = screen_width()-ball.radius;
                ball.velocity.x *= -ball_to_wall_c;
                
            }

            //floor and ceiling
            if ball.position.y < ball.radius {
                ball.position.y = ball.radius;
                ball.velocity.y *= -ball_to_wall_c;
                
            } else if ball.position.y > screen_height()-ball.radius {
                ball.position.y = screen_height()-ball.radius;
                ball.velocity.y *= -ball_to_wall_c;
                
            }
        }

        
        for _substep in 1..=10 {
            //resolve ball-to-ball collisions
            let ball_count = balls.len();
            let indices = 0..ball_count;

            // For each element in the range from 0 to ball count it maps it to a range from it's current index to ball count
            // This creates a pairing between every index
            let index_pairs = indices
                .map(move |i| ((i + 1)..ball_count).map(move |j| (i, j)))
                .flatten(); // Flatten converts an iterator of iterators into a single iterator. Matrix -> array.

            for (i, j) in index_pairs {
                if check_collides(&balls[i], &balls[j]) { // note: can pass these by copy too
                    
                    match balls.get_many_mut([i, j]) {
                        Ok([ball_1, ball_2]) => ball_to_ball_collision(ball_1, ball_2, ball_to_ball_c),
                        Err(_err) => {
                            // Index i = index j; in this case no collision occurs!
                            
                        }
                    }
                }
            }
        }



        //draw
        for ball in balls.clone().into_iter() {
            draw_circle(ball.position.x, ball.position.y, ball.radius, WHITE);
        }
        next_frame().await
    }
}


//bug fixing
// decrease gravity and velocities.

// [ ] mouse click to spawn them
// [ ] randomise size too

//println!\(".*?\);\s*

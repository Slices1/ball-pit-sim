#![feature(get_many_mut)]
use macroquad::prelude::*;
use rayon::prelude::*;

#[derive(Clone)]
struct Ball {
    radius: f32,
    position: Vec2,
    velocity: Vec2,
    colour: Color,
}

fn spawn_ball(position: Vec2, balls: &mut Vec<Ball>) {
    
    balls.push(Ball {
        radius: rand::gen_range(1.5, 1.5),
        position: position,
        velocity: Vec2{x:0.,y:0.},
        colour: WHITE,
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

pub fn random_pastel() -> Color {

    // Generate pastel colors (light colors with high values)
    let r = rand::gen_range(0.6, 1.);
    let g = rand::gen_range(0.6, 1.);
    let b = rand::gen_range(0.6, 1.);

    return Color::new(r, g, b, 1.0);
}

#[macroquad::main("Ball Pit Sim in Rust")]
async fn main() {
    set_fullscreen(true);
    for _i in 1..=20 { // so full screen has time to open
        clear_background(GRAY);
        next_frame().await;
    }

    let mut balls: Vec<Ball> = Vec::new();
    
    let gravity: f32 = 0.0;
    let ball_to_wall_c: f32 = 0.8;
    let ball_to_ball_c: f32 = 0.8;

    // Spawning method #1 - all at once
    for i in 1..=80 {
        for j in 1..=40 {
            spawn_ball(Vec2{x:30. + 10.*(i as f32),y:screen_height() -30. - 10.*(j as f32)}, &mut balls);
        }
    }

    //balls[24].position.x += 1.;

    // colour them
    for ball in &mut balls {
        ball.colour = random_pastel();
        ball.velocity = Vec2{x:rand::gen_range(-70., 70.),y:rand::gen_range(-70., 70.)};
    }

    let mut frame_number: u64 = 0;
    let mut ball_count = balls.len();
    loop {
        clear_background(GRAY);
        //spawn
        // Spawning method #2 - horizontal sinusoidal motion
        // frame_number+=1;
        // if frame_number % 4 == 0 {
        //     spawn_ball(Vec2{x:screen_width()*((frame_number as f32/50.).sin()+1.)/2.,y:100.}, &mut balls);
        //     ball_count += 1;
        // }

        if (is_mouse_button_pressed(MouseButton::Left)) {
            spawn_ball(mouse_position().into(), &mut balls);
            ball_count += 1;
            balls[ball_count-1].colour = random_pastel();
            balls[ball_count-1].radius = 40.;
        }

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

        //resolve ball-to-ball collisions
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

        //draw
        for ball in balls.clone().into_iter() {
            draw_circle(ball.position.x, ball.position.y, ball.radius, ball.colour);
        }
        next_frame().await
    }
}

// feaures to potentiall add:
// [X] mouse click to spawn them
// [ ] parallelize the collision logic
// [X] randomise size

//regex for prints: println!\(".*?\);\s*

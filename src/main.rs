use std::f32::consts::PI;

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: Vec3,
}

//new keyword is considered naming convention for general constructor
impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
        }
    }
}

fn cube_vertices() -> Vec<Vertex> {
    vec![
        // Front face
        Vertex::new(0.0, 0.0, 0.0),
        Vertex::new(1.0, 0.0, 0.0),
        Vertex::new(1.0, 1.0, 0.0),
        Vertex::new(0.0, 1.0, 0.0),
        // Back face
        Vertex::new(0.0, 0.0, -1.0),
        Vertex::new(1.0, 0.0, -1.0),
        Vertex::new(1.0, 1.0, -1.0),
        Vertex::new(0.0, 1.0, -1.0),
    ]
}
//comment here
fn cube_indices() -> Vec<u16> {
    vec![
        0, 1, 2, 2, 3, 0, // Front face
        4, 5, 6, 6, 7, 4, // Back face
        0, 4, 7, 7, 3, 0, // Left face
        1, 5, 6, 6, 2, 1, // Right face
        3, 2, 6, 6, 7, 3, // Top face
        0, 1, 5, 5, 4, 0, // Bottom face
    ]
}

fn populate_grid(GRID_SIZE: i32, colour_list_size: i32) -> Vec<(i32, i32, i32)> {
    let mut visited: Vec<(i32, i32, i32)> = vec![];
    rand::srand(macroquad::miniquad::date::now() as _);
    for i in -(GRID_SIZE / 2)..(GRID_SIZE / 2) {
        for j in -(GRID_SIZE / 2) + 1..(GRID_SIZE / 2) + 1 {
            visited.push((i, j, rand::gen_range(0, colour_list_size + 1)));
        }
    }
    return visited;
}

#[macroquad::main("Rolling Cube")]
async fn main() {
    let vertices = cube_vertices();
    let indices = cube_indices();
    const GRID_SIZE: i32 = 8; //square grid side length
    let mut count = 0; //movement counter
    let mut roll_speed: f32 = 25.0; //roll speed
    let mut x_roll_angle: f32 = 0.0; //the way movement is simulated in x direction (3d space)
    let mut z_roll_angle: f32 = 0.0; //the way movement is simulated in z direction (3d space)
    let mut xdir_offset: f32 = 0.0; //treated as y-coord value in relation to flat grid (2d space)
    let mut zdir_offset: f32 = 0.0; //treated as x-coord value in relation to flat grid (2d space)
    let mut xdir_offset_smooth: f32 = 0.0; //smoothed xdir_offset value for camera movement
    let mut zdir_offset_smooth: f32 = 0.0; //smoothed zdir_offset value for camera movement
    let mut up_flag: bool = false; //up arrow key pressed
    let mut down_flag: bool = false; //down arrow key pressed
    let mut right_flag: bool = false; //right arrow key pressed
    let mut left_flag: bool = false; //left arrow key pressed
    let mut rotate_flag: bool = false; //cube in process of rotating
    let mut stationary: bool = false; //cube is stationary. (similar, but not the same to rotate_flag: evaluated and used at different stage of rotation)
                                      //let mut visited: Vec<(i32, i32, i32)> = vec![]; //a vector of coords visited including incrementing counter
    let colour_list: Vec<Color> = vec![RED];
    let colour_list2: Vec<Color> = vec![RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE, VIOLET, PINK];
    //initial population of "tiles"
    // rand::srand(macroquad::miniquad::date::now() as _);
    // for i in -(GRID_SIZE / 2)..(GRID_SIZE / 2) {
    //     for j in -(GRID_SIZE / 2) + 1..(GRID_SIZE / 2) + 1 {
    //         visited.push((i, j, rand::gen_range(0, colour_list.len() as i32 + 1)));
    //     }
    // }
    let mut visited: Vec<(i32, i32, i32)> = populate_grid(GRID_SIZE, colour_list.len() as i32);
    let mut win_flag: bool = false;

    //main program loop begins
    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Space) {
            win_flag = false;
            visited.clear();
            visited = populate_grid(GRID_SIZE, colour_list.len() as i32);
        }

        if (visited.iter().filter(|x| x.2 != 0).count() == 0) {
            win_flag = true;
        }
        if (win_flag) {
            set_default_camera();
            let textcen = get_text_center("YOU WIN", Option::None, 80, 1.0, 0.0);
            draw_text(
                "YOU WIN!",
                screen_width() / 2.0 - textcen.x,
                screen_height() / 2.0 - textcen.y,
                80.0,
                DARKBLUE,
            );
            let textcen2 = get_text_center(
                format!("Moves taken: {}", count).as_str(),
                Option::None,
                80,
                1.0,
                0.0,
            );
            draw_text(
                format!("Moves taken: {}", count).as_str(),
                screen_width() / 2.0 - textcen2.x,
                screen_height() / 10.0 * 4.0 - textcen2.y,
                80.0,
                DARKBLUE,
            );
        }
        set_camera(&Camera3D {
            position: vec3(0.0, 4., xdir_offset_smooth - 5.0),
            up: vec3(0., 1., 0.),
            target: vec3(zdir_offset_smooth, 0., xdir_offset_smooth),
            ..Default::default()
        });
        draw_grid(GRID_SIZE as u32, 1., GRAY, GRAY);

        if is_key_pressed(KeyCode::Q) {
            roll_speed += 1.0;
        }
        if is_key_pressed(KeyCode::A) {
            roll_speed -= 1.0;
        }

        //note: the down and left keys had to be treated differently to the up and right keys.
        //this is due to the way the rotation is handled
        //rotation flag allows continuous key press without issue
        match (
            !rotate_flag,
            is_key_down(KeyCode::Up),
            is_key_down(KeyCode::Down),
            is_key_down(KeyCode::Right),
            is_key_down(KeyCode::Left),
        ) {
            (true, true, _, _, _) if xdir_offset < 4.0 => {
                up_flag = true;
                rotate_flag = true;
                x_roll_angle = 0.0;
            }
            (true, _, true, _, _) if xdir_offset > -3.0 => {
                down_flag = true;
                rotate_flag = true;
                x_roll_angle = PI / 2.0;
                xdir_offset -= 1.0;
                stationary = false;
            }
            (true, _, _, true, _) if zdir_offset > -4.0 => {
                right_flag = true;
                rotate_flag = true;
                z_roll_angle = 0.0;
            }
            (true, _, _, _, true) if zdir_offset < 3.0 => {
                left_flag = true;
                rotate_flag = true;
                z_roll_angle = PI / 2.0;
                zdir_offset += 1.0;
                stationary = false;
            }
            _ => {}
        }
        if up_flag {
            stationary = false;
            x_roll_angle += PI / roll_speed;
            xdir_offset_smooth += 1.0 / (roll_speed);
            if x_roll_angle >= PI / 2.0 {
                x_roll_angle = 0.0;
                up_flag = false;
                xdir_offset += 1.0;
            }
            if x_roll_angle == 0.0 {
                count += 1;
                stationary = true;
                rotate_flag = false;
            }
        }
        if down_flag {
            x_roll_angle -= PI / roll_speed;
            xdir_offset_smooth -= 1.0 / (roll_speed);
            if x_roll_angle <= 0.0 {
                x_roll_angle = PI / 2.0;
                down_flag = false;
            }
            if x_roll_angle == PI / 2.0 {
                count += 1;
                stationary = true;
                rotate_flag = false;
            }
        }
        if left_flag {
            z_roll_angle -= PI / roll_speed;
            zdir_offset_smooth += 1.0 / (roll_speed);

            if z_roll_angle <= 0.0 {
                z_roll_angle = PI / 2.0;
                left_flag = false;
            }
            if z_roll_angle == PI / 2.0 {
                count += 1;
                stationary = true;
                rotate_flag = false;
            }
        }
        if right_flag {
            stationary = false;
            z_roll_angle += PI / roll_speed;
            zdir_offset_smooth -= 1.0 / (roll_speed);
            if z_roll_angle >= PI / 2.0 {
                z_roll_angle = 0.0;
                right_flag = false;
                zdir_offset -= 1.0;
            }
            if z_roll_angle == 0.0 {
                count += 1;
                stationary = true;
                rotate_flag = false;
            }
        }
        //for all coords in the grid: if the xdir_offset and zdir_offset are equal to their respective coord (and cube is stationary)
        for i in 0..(visited.len()) {
            if visited[i].0 == zdir_offset as i32
                && visited[i].1 == xdir_offset as i32
                && stationary == true
            {
                if visited[i].2 >= 0 && visited[i].2 < (colour_list.len()) as i32 {
                    //if visit count is greater than or equal to 0 but less than the length of the colour list
                    visited[i].2 += 1; //increment visit count
                    stationary = false;
                } else if visited[i].2 == (colour_list.len()) as i32 {
                    //otherwise
                    visited[i].2 = 0; //reset visit count
                    stationary = false;
                }

                println!(
                    "x,y({},{})--speed {}-- {}----- moves: {}--------{}",
                    visited[i].0,
                    visited[i].1,
                    roll_speed,
                    visited[i].2,
                    count,
                    visited.len()
                );
            }
            //render a small plane ("tile") with its colour based on visit count
            if visited[i].2 >= 1 {
                draw_plane(
                    vec3(visited[i].0 as f32 + 0.5, 0.0, visited[i].1 as f32 - 0.5),
                    vec2(0.5, 0.5),
                    None,
                    colour_list[visited[i].2 as usize - 1],
                );
            }
        }
        // rotates the model depending on up/down/right/left flag
        let mut model = Mat4::from_translation(vec3(zdir_offset, 0.0, xdir_offset));
        if up_flag || down_flag {
            model = model * Mat4::from_rotation_x(x_roll_angle);
        } else if right_flag || left_flag {
            model = model * Mat4::from_rotation_z(z_roll_angle);
        }
        for i in (0..indices.len()).step_by(3) {
            //interates through indices list
            let i0 = indices[i] as usize;
            let i1 = indices[i + 1] as usize;
            let i2 = indices[i + 2] as usize;

            let v0 = model //vertex set 1
                * vec4(
                    vertices[i0].position.x,
                    vertices[i0].position.y,
                    vertices[i0].position.z,
                    1.0,
                );
            let v1 = model //vertex set 2
                * vec4(
                    vertices[i1].position.x,
                    vertices[i1].position.y,
                    vertices[i1].position.z,
                    1.0,
                );
            let v2 = model //vertex set 3
                * vec4(
                    vertices[i2].position.x,
                    vertices[i2].position.y,
                    vertices[i2].position.z,
                    1.0,
                );
            draw_line_3d(v0.truncate(), v1.truncate(), YELLOW); //render line from vertex set 1 to vertex set 2
            draw_line_3d(v1.truncate(), v2.truncate(), YELLOW); //render line from vertex set 2 to vertex set 3
                                                                //draw_line_3d(v0.truncate(), v2.truncate(), cube_colour); //render line from vertex set 2 to vertex set 3
        }
        set_default_camera();
        let textcen = get_text_center(
            format!("Moves taken: {}", count).as_str(),
            Option::None,
            40,
            1.0,
            0.0,
        );
        draw_text(
            format!("Moves taken: {}", count).as_str(),
            screen_width() / 2.0 - textcen.x,
            screen_height() / 12.0 - textcen.y,
            40.0,
            DARKBLUE,
        );
        draw_text(
            format!(
                "squares remaining: {}",
                visited.iter().filter(|x| x.2 != 0).count()
            )
            .as_str(),
            screen_width() / 2.0 - textcen.x,
            screen_height() / 12.0 * 1.8 - textcen.y,
            40.0,
            DARKBLUE,
        );
        next_frame().await
    }
}
//"ONLY THE DEAD KNOW PEACE FROM THIS SUFFERING",

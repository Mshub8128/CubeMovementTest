use std::f32::consts::PI;

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: Vec3,
}

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

#[macroquad::main("Rolling Cube")]
async fn main() {
    let vertices = cube_vertices();
    let indices = cube_indices();
    let grid_size: i32 = 8; //square grid side length
    let mut count = 0; //movement counter
    let mut roll_speed: f32 = 85.0; //roll speed
    let mut x_roll_angle: f32 = 0.0; //the way movement is simulated in x direction (3d space)
    let mut z_roll_angle: f32 = 0.0; //the way movement is simulated in z direction (3d space)
                                     // let mut horiz: f32 = 0.0; //PI / 2.0; //camera position/ rotation around centre
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
    let mut visited: Vec<(i32, i32, i32)> = vec![]; //a vector of coords visited including incrementing counter
    let colour_list: Vec<Color> = vec![RED];
    let colour_list2: Vec<Color> = vec![RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE, VIOLET, PINK];
    //initial population of "tiles"
    for i in -(grid_size / 2)..(grid_size / 2) {
        for j in -(grid_size / 2) + 1..(grid_size / 2) + 1 {
            visited.push((i, j, rand::gen_range(0, colour_list.len() as i32 + 1)));
        }
    }
    let mut cube_colour: Color;

    //main program loop begins
    loop {
        clear_background(LIGHTGRAY);
        draw_grid(grid_size as u32, 1., GRAY, GRAY);

        set_camera(&Camera3D {
            position: vec3(0.0, 4., xdir_offset_smooth - 5.0),
            up: vec3(0., 1., 0.),
            target: vec3(zdir_offset_smooth, 0., xdir_offset_smooth),
            ..Default::default()
        });

        // if is_key_down(KeyCode::LeftShift) {
        //     horiz -= PI / 540.0;
        // }
        // if is_key_down(KeyCode::RightShift) {
        //     horiz += PI / 540.0;
        // }
        if is_key_pressed(KeyCode::Q) {
            roll_speed += 1.0;
        }
        if is_key_pressed(KeyCode::A) {
            roll_speed -= 1.0;
        }

        //note: the down and left keys had to be treated differently to the up and right keys.
        //this is due to the way the rotation is handled
        if !rotate_flag {
            if is_key_down(KeyCode::Up) && xdir_offset < 4.0 {
                up_flag = true;
                rotate_flag = true;
                x_roll_angle = 0.0;
            }

            if is_key_down(KeyCode::Down) && xdir_offset > -3.0 {
                down_flag = true;
                rotate_flag = true;
                x_roll_angle = PI / 2.0;
                xdir_offset -= 1.0;
                stationary = false;
            }

            if is_key_down(KeyCode::Right) && zdir_offset > -4.0 {
                right_flag = true;
                rotate_flag = true;
                z_roll_angle = 0.0;
            }

            if is_key_down(KeyCode::Left) && zdir_offset < 3.0 {
                left_flag = true;
                rotate_flag = true;
                z_roll_angle = PI / 2.0;
                zdir_offset += 1.0;
                stationary = false;
            }
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
        for i in 1..(visited.len()) {
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
                    "x,y({},{})--speed {}-- {}----- moves: {}",
                    zdir_offset, xdir_offset, roll_speed, visited[i].2, count
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
            cube_colour = colour_list2[(xdir_offset + 3.0) as usize]; //change cube colour (purely for fun)
            draw_line_3d(v0.truncate(), v1.truncate(), cube_colour); //render line from vertex set 1 to vertex set 2
            draw_line_3d(v1.truncate(), v2.truncate(), cube_colour); //render line from vertex set 2 to vertex set 3
        }

        next_frame().await
    }
}

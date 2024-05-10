use std::f32::consts::PI;
use std::vec;

use macroquad::color;
use macroquad::prelude::*;
//use macroquad::text;

//#[derive(Debug, Clone, Copy)]
#[macroquad::main("Rolling Cube")]
async fn main() {
    //Coordinates of the vertices of a cube in 3D space
    let vertices = [
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
    ];
    //the vertices required to render each face of the cube
    let indices = [
        0, 1, 2, 2, 3, 0, // Front face
        4, 5, 6, 6, 7, 4, // Back face
        0, 4, 7, 7, 3, 0, // Left face
        1, 5, 6, 6, 2, 1, // Right face
        3, 2, 6, 6, 7, 3, // Top face
        0, 1, 5, 5, 4, 0, // Bottom face
    ];

    const GRID_SIZE: i32 = 7; //square grid side length
    let mut count = 0; //movement counter
    let roll_speed: f32 = 45.0; //roll speed
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
    let mut end_flag: bool = false; //end state
    let end_value: i32 = (GRID_SIZE + GRID_SIZE % 2) * (GRID_SIZE + GRID_SIZE % 2) * 2; //number of allowable moves
    let mut high_score: i32 = 0; //... high score
                                 //vector of tile colours. in current implementation of code, only one colour is required.
    let tile_colours: Vec<color::Color> = vec![macroquad::color::Color::from_vec(vec4(
        MAROON.r,
        MAROON.g,
        MAROON.b,
        MAROON.a / 3.0,
    ))];
    //vector of cube colours. gives indication of how many moves left.
    let cube_colours: Vec<color::Color> = vec![GREEN, YELLOW, ORANGE, RED, BLANK];
    //vector of squares in grid and number of times visited.
    let mut visited: Vec<(i32, i32, i32)> = populate_grid(GRID_SIZE, tile_colours.len() as i32);
    //main program loop begins
    loop {
        clear_background(DARKGRAY);
        let cube_colour_current: Color = cube_colours[count as usize / (end_value as usize / 4)];

        //Reset button
        if is_key_down(KeyCode::Space) {
            end_flag = false;
            visited.clear();
            visited = populate_grid(GRID_SIZE, tile_colours.len() as i32);
            xdir_offset = 0.0;
            zdir_offset = 0.0;
            xdir_offset_smooth = 0.0;
            zdir_offset_smooth = 0.0;
            count = 0;
        }
        //check end conditions
        if (visited.iter().filter(|x| x.2 != 0).count() == 0) || count >= end_value {
            end_flag = true;
        }
        if end_flag && count < end_value {
            if high_score > count || high_score == 0 {
                high_score = count;
            }

            set_default_camera();
            text_helper(false, "YOU WIN!", None, 80.0, 0.5, 0.5, GOLD);
            text_helper(true, "Moves taken: ", Some(count), 80.0, 0.5, 0.6, GOLD);
        } else if end_flag && count >= end_value {
            set_default_camera();
            text_helper(false, ":( YOU LOSE :(", None, 80.0, 0.5, 0.5, GOLD);
            text_helper(
                true,
                "Squares left:",
                Some(
                    visited
                        .iter()
                        .filter(|x| x.2 != 0)
                        .count()
                        .try_into()
                        .unwrap(),
                ),
                80.0,
                0.5,
                7.0 / 10.0,
                GOLD,
            );
        } else {
            set_default_camera();
            text_helper(
                true,
                "Moves taken:",
                Some(count),
                40.0,
                0.5,
                1.0 / 12.0,
                cube_colour_current,
            );
            text_helper(
                true,
                "Squares remaining:",
                Some(
                    visited
                        .iter()
                        .filter(|x| x.2 != 0)
                        .count()
                        .try_into()
                        .unwrap(),
                ),
                40.0,
                0.5,
                1.8 / 12.0,
                GOLD,
            );
            if high_score != 0 {
                text_helper(
                    true,
                    "High score:",
                    Some(high_score),
                    20.0,
                    5.0 / 10.0,
                    0.8 / 25.0,
                    GOLD,
                );
            }
        }
        text_helper(
            true,
            "Grid size:",
            Some(GRID_SIZE),
            20.0,
            8.5 / 10.0,
            1. / 25.,
            GREEN,
        );
        text_helper(
            true,
            "Max moves:",
            Some(end_value),
            20.0,
            8.5 / 10.0,
            2. / 25.,
            GREEN,
        );
        text_helper(
            false,
            "Click 'space' to restart.",
            None,
            20.0,
            1.5 / 10.0,
            2.0 / 25.0,
            GREEN,
        );
        text_helper(
            false,
            "Use arrow keys to move.",
            None,
            20.0,
            1.5 / 10.0,
            1.0 / 25.0,
            GREEN,
        );
        set_camera(&Camera3D {
            position: vec3(0.5, 4., xdir_offset_smooth - 5.0),
            up: vec3(0., 1., 0.),
            target: vec3(zdir_offset_smooth + 0.5, 0., xdir_offset_smooth),
            ..Default::default()
        });

        //note: the down and left keys had to be treated differently to the up and right keys.
        //this is due to the way the rotation is handled
        //rotation flag allows continuous key press without issue
        match (
            !end_flag,
            !rotate_flag,
            is_key_down(KeyCode::Up),
            is_key_down(KeyCode::Down),
            is_key_down(KeyCode::Right),
            is_key_down(KeyCode::Left),
        ) {
            (true, true, true, _, _, _) if xdir_offset < (GRID_SIZE / 2) as f32 => {
                up_flag = true;
                rotate_flag = true;
                x_roll_angle = 0.0;
            }
            (true, true, _, true, _, _)
                if xdir_offset > (-(GRID_SIZE / 2) - (GRID_SIZE % 2 - 1)) as f32 =>
            {
                down_flag = true;
                rotate_flag = true;
                x_roll_angle = PI / 2.0;
                xdir_offset -= 1.0;
                stationary = false;
            }
            (true, true, _, _, true, _) if zdir_offset > -(GRID_SIZE / 2) as f32 => {
                right_flag = true;
                rotate_flag = true;
                z_roll_angle = 0.0;
            }
            (true, true, _, _, _, true)
                if zdir_offset < ((GRID_SIZE / 2) + (GRID_SIZE % 2 - 1)) as f32 =>
            {
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
                && stationary
            {
                if visited[i].2 >= 0 && visited[i].2 < (tile_colours.len()) as i32 {
                    //if visit count is greater than or equal to 0 but less than the length of the colour list
                    visited[i].2 += 1; //increment visit count
                    stationary = false;
                } else if visited[i].2 == (tile_colours.len()) as i32 {
                    //otherwise
                    visited[i].2 = 0; //reset visit count
                    stationary = false;
                }
            }
            //render a small plane ("tile") with its colour based on visit count
            let x = visited[i].0 as f32;
            let z = visited[i].1 as f32;

            //draw tiles
            if visited[i].2 >= 1 {
                draw_plane(
                    vec3(x + 0.5, 0.0, z - 0.5),
                    vec2(0.5, 0.5),
                    None,
                    tile_colours[visited[i].2 as usize - 1],
                );
            } else {
                draw_plane(vec3(x + 0.5, 0.0, z - 0.5), vec2(0.5, 0.5), None, BLANK);
            }
            draw_line_3d(vec3(x, 0.0, z), vec3(x + 1.0, 0.0, z), BLACK);
            draw_line_3d(vec3(x, 0.0, z), vec3(x, 0.0, z - 1.0), BLACK);
            draw_line_3d(vec3(x + 1.0, 0.0, z), vec3(x + 1.0, 0.0, z - 1.0), BLACK);
            draw_line_3d(vec3(x, 0.0, z - 1.0), vec3(x + 1.0, 0.0, z - 1.0), BLACK);
        }
        // rotates the model depending on up/down/right/left flag
        let mut model = Mat4::from_translation(vec3(zdir_offset, 0.0, xdir_offset));
        if up_flag || down_flag {
            model *= Mat4::from_rotation_x(x_roll_angle);
        } else if right_flag || left_flag {
            model *= Mat4::from_rotation_z(z_roll_angle);
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

            draw_line_3d(v0.truncate(), v1.truncate(), cube_colour_current); //render line from vertex set 1 to vertex set 2
            draw_line_3d(v1.truncate(), v2.truncate(), cube_colour_current);
        }
        next_frame().await
    }
}

struct Vertex {
    position: Vec3,
}

//new keyword is considered naming convention for general constructor in Rust
impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
        }
    }
}
//function to randomly populate grid
fn populate_grid(grid_size: i32, tile_colours_size: i32) -> Vec<(i32, i32, i32)> {
    let mut visited: Vec<(i32, i32, i32)> = vec![];
    rand::srand(macroquad::miniquad::date::now() as _);
    if grid_size % 2 == 0 {
        for i in -(grid_size / 2)..(grid_size / 2) {
            for j in -(grid_size / 2) + 1..(grid_size / 2) + 1 {
                visited.push((i, j, rand::gen_range(0, tile_colours_size + 1)));
            }
        }
        visited
    } else {
        for i in -(grid_size / 2)..(grid_size / 2) + 1 {
            for j in -(grid_size / 2)..(grid_size / 2) + 1 {
                visited.push((i, j, rand::gen_range(0, tile_colours_size + 1)));
            }
        }
        visited
    }
}

//function to more easily call get_text_center and draw_text functions
fn text_helper<T: AsRef<str>>(
    incl_var: bool,
    text: T,
    text_var: Option<i32>,
    size: f32,
    pos_x: f32,
    pos_y: f32,
    text_colour: Color,
) {
    let full_text = if incl_var {
        format!("{} {}", text.as_ref(), text_var.unwrap_or(0))
    } else {
        text.as_ref().to_string()
    };

    let pos = get_text_center(&full_text, None, size as u16, 1.0, 0.0);
    draw_text(
        &full_text,
        screen_width() * pos_x - pos.x,
        screen_height() * pos_y - pos.y,
        size,
        text_colour,
    );
}

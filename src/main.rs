use std::{f32::consts::PI, vec};

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

    let mut roll_speed: f32 = 30.0;
    let mut x_roll_angle: f32 = 0.0;
    let mut z_roll_angle: f32 = 0.0;
    let mut horiz: f32 = PI / 8.0;
    let mut xdir_offset: f32 = 0.0;
    let mut zdir_offset: f32 = 0.0;
    // let mut xdir_offset_prev: f32 = 0.0;
    // let mut zdir_offset_prev: f32 = 0.0;
    let mut up_flag: bool = false;
    let mut down_flag: bool = false;
    let mut right_flag: bool = false;
    let mut left_flag: bool = false;
    let mut rotate_flag: bool = false;
    let mut stationary: bool = false;
    let mut visited: Vec<(i32, i32, i32)> = vec![];
    for i in -10..10 {
        for j in -10..10 {
            visited.push((i, j, 0));
        }
    }
    let colour_list: Vec<Color> = vec![GREEN, YELLOW, RED];
    loop {
        clear_background(LIGHTGRAY);
        draw_grid(8, 1., BLACK, GRAY);

        set_camera(&Camera3D {
            position: vec3(-7.0 * horiz.sin(), 4., -7.0 * horiz.cos() - 0.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        if is_key_down(KeyCode::LeftShift) {
            horiz -= PI / 540.0;
        }
        if is_key_down(KeyCode::RightShift) {
            horiz += PI / 540.0;
        }
        if is_key_pressed(KeyCode::Q) {
            roll_speed += 1.0;
        }
        if is_key_pressed(KeyCode::A) {
            roll_speed -= 1.0;
        }

        if !rotate_flag {
            if is_key_pressed(KeyCode::Up)
                || is_key_pressed(KeyCode::Down)
                || is_key_pressed(KeyCode::Right)
                || is_key_pressed(KeyCode::Left)
            {
                // xdir_offset_prev = xdir_offset;
                // zdir_offset_prev = zdir_offset;
            }

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
            if x_roll_angle >= PI / 2.0 {
                x_roll_angle = 0.0;
                up_flag = false;
                xdir_offset += 1.0;
            }
            if x_roll_angle == 0.0 {
                stationary = true;
                rotate_flag = false;
            }
        }
        if down_flag {
            x_roll_angle -= PI / roll_speed;
            if x_roll_angle <= 0.0 {
                x_roll_angle = PI / 2.0;
                down_flag = false;
            }
            if x_roll_angle == PI / 2.0 {
                stationary = true;
                rotate_flag = false;
            }
        }
        if left_flag {
            z_roll_angle -= PI / roll_speed;
            if z_roll_angle <= 0.0 {
                z_roll_angle = PI / 2.0;
                left_flag = false;
            }
            if z_roll_angle == PI / 2.0 {
                stationary = true;
                rotate_flag = false;
            }
        }
        if right_flag {
            stationary = false;

            z_roll_angle += PI / roll_speed;
            if z_roll_angle >= PI / 2.0 {
                z_roll_angle = 0.0;
                right_flag = false;
                zdir_offset -= 1.0;
            }
            if z_roll_angle == 0.0 {
                stationary = true;
                rotate_flag = false;
            }
        }

        for i in 1..(visited.len()) {
            if visited[i].0 == zdir_offset as i32
                && visited[i].1 == xdir_offset as i32
                && stationary == true
            {
                if visited[i].2 == 0 {
                    visited[i].2 = 1;
                    stationary = false;
                } else if visited[i].2 == 1 {
                    visited[i].2 = 2;
                    stationary = false;
                } else if visited[i].2 == 2 {
                    visited[i].2 = 3;
                    stationary = false;
                } else if visited[i].2 == 3 {
                    visited[i].2 = 0;
                    stationary = false;
                }

                println!(
                    "x,z({},{})--speed {}-- {}",
                    zdir_offset, xdir_offset, roll_speed, visited[i].2
                );
            }

            if visited[i].2 >= 1 {
                draw_plane(
                    vec3(visited[i].0 as f32 + 0.5, 0.0, visited[i].1 as f32 - 0.5),
                    vec2(0.5, 0.5),
                    None,
                    colour_list[visited[i].2 as usize - 1],
                );
            }
        }

        let mut model = Mat4::from_translation(vec3(zdir_offset, 0.0, xdir_offset));
        if up_flag || down_flag {
            model = model * Mat4::from_rotation_x(x_roll_angle);
        } else if right_flag || left_flag {
            model = model * Mat4::from_rotation_z(z_roll_angle);
        }
        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize;
            let i1 = indices[i + 1] as usize;
            let i2 = indices[i + 2] as usize;

            let v0 = model
                * vec4(
                    vertices[i0].position.x,
                    vertices[i0].position.y,
                    vertices[i0].position.z,
                    1.0,
                );
            let v1 = model
                * vec4(
                    vertices[i1].position.x,
                    vertices[i1].position.y,
                    vertices[i1].position.z,
                    1.0,
                );
            let v2 = model
                * vec4(
                    vertices[i2].position.x,
                    vertices[i2].position.y,
                    vertices[i2].position.z,
                    1.0,
                );

            draw_line_3d(v0.truncate(), v1.truncate(), RED);
            draw_line_3d(v1.truncate(), v2.truncate(), RED);
        }

        next_frame().await
    }
}

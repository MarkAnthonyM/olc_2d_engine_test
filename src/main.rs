extern crate winapi;

use std::ptr;
use winapi::ctypes;
use winapi::shared::ntdef;
use winapi::um::{ wincon, winuser, winnt, wincontypes };
use std::time::{ Instant };
use widestring::U16CString;

fn main() {
    let buff_width = 120;
    let buff_height = 40;

    let mut player_x = 8.0;
    let mut player_y = 8.0;
    let mut player_a = 0.0;

    const MAP_HEIGHT: u32 = 16;
    const MAP_WIDTH: u32 = 16;

    let player_fov = 3.14159 / 4.0;
    let depth = 16.0;

    let mut map = String::new();

    map += "################";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..........#...#";
    map += "#..........#...#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#.......########";
    map += "#..............#";
    map += "#..............#";
    map += "################";

    let map_slice = &map.as_bytes(); //index-able reference slice

    let buff_coord = wincontypes::COORD {
        X: 0,
        Y: 0,
    };

    let mut window_buffer: Vec<ctypes::wchar_t> = vec!['*' as u16; buff_width * buff_height];
    let buffer_ptr = window_buffer.as_ptr();
    let buff_sec_ptr = window_buffer.as_mut_ptr();

    let mut dw_bytes_written = 0;

    let mut time_point_1 = Instant::now();
    let mut time_point_2;

    let mut index_val = player_y as u32 * MAP_WIDTH + player_x as u32;
    let mut index_val_con = index_val as usize;

    unsafe {
        let hconsole = wincon::CreateConsoleScreenBuffer(winnt::GENERIC_READ | winnt::GENERIC_WRITE, 0, ptr::null(), wincon::CONSOLE_TEXTMODE_BUFFER, ntdef::NULL);

        wincon::SetConsoleActiveScreenBuffer(hconsole);


        loop {
            //Duration measurement to time frame speed
            time_point_2 = Instant::now();
            let elapsed_time = time_point_2.duration_since(time_point_1);
            let in_nano = elapsed_time.as_micros() as f64 / 100_000.0;
            time_point_1 = time_point_2;

            //Controls
            //Handle CCW Rotation
            let key_trigger_a = winuser::GetAsyncKeyState('A' as i32);
            let key_trigger_d = winuser::GetAsyncKeyState('D' as i32);
            let key_trigger_w = winuser::GetAsyncKeyState('W' as i32);
            let key_trigger_s = winuser::GetAsyncKeyState('S' as i32);

            if key_trigger_a != 0 && key_trigger_a == -32768 {
                player_a -= 0.1 * in_nano;
            }

            if key_trigger_d != 0 && key_trigger_d == -32768 {
                player_a += 0.1 * in_nano;
            }

            if key_trigger_w != 0 && key_trigger_w == -32768 {
                player_x += player_a.sin() * 0.5 * in_nano;
                player_y += player_a.cos() * 0.5 * in_nano;

                index_val = player_y as u32 * MAP_WIDTH + player_x as u32;
                index_val_con = index_val as usize;

                if map_slice[index_val_con] as char == '#' {
                    player_x -= player_a.sin() * 0.5 * in_nano;
                    player_y -= player_a.cos() * 0.5 * in_nano;
                }
            }

            if key_trigger_s != 0 && key_trigger_s == -32768 {
                player_x -= player_a.sin() * 0.5 * in_nano;
                player_y -= player_a.cos() * 0.5 * in_nano;

                index_val = player_y as u32 * MAP_WIDTH + player_x as u32;
                index_val_con = index_val as usize;

                if map_slice[index_val_con] as char == '#' {
                    player_x += player_a.sin() * 0.5 * in_nano;
                    player_y += player_a.cos() * 0.5 * in_nano;
                }
            }

            //Displays player coordinates on screen
            let w = format!("X: {:.2} Y: {:.2} A: {:.2}", &player_x, &player_y, &player_a);
            let w_string = U16CString::from_str(w).unwrap();
            let s_ptr = w_string.as_ptr();

            for i in 0..buff_width {
                //For each column, calculate the projected ray angle into world space
                let ray_angle = (player_a - player_fov / 2.0) + (i as f64 / buff_width as f64) * player_fov;

                // Find distance to wall
                let mut distance_to_wall = 0.0;
                let mut hit_wall = false;

                let eye_x = ray_angle.sin(); //Unit vector for ray in player space
                let eye_y = ray_angle.cos();

                while !hit_wall && distance_to_wall < depth {
                    distance_to_wall += 0.1;

                    let test_x = player_x + eye_x * distance_to_wall;
                    let test_y = player_y + eye_y * distance_to_wall;

                    let test_x_con = test_x as i32;
                    let test_y_con = test_y as i32;

                    //Test if ray is out of bounds
                    if test_x_con < 0 || test_x_con >= MAP_WIDTH as i32 || test_y_con < 0 || test_y_con >= MAP_HEIGHT as i32 {
                        hit_wall = true; //Just set distance to maximum depth
                        distance_to_wall = depth;
                    } else {
                        // Ray is inbounds so test to see if the ray cell is a wall block
                        let test_convert = test_y_con * MAP_WIDTH as i32 + test_x_con;
                        let test_con = test_convert as usize; //temporary cast for arithmetic sake
                        if map_slice[test_con] as char == '#' {
                            hit_wall = true;
                        }
                    }
                }

                //Calculate distance to ceiling and floor
                let ceiling = (buff_height as f64 / 2.0) - buff_height as f64 / distance_to_wall;
                let ceiling_con = ceiling as isize; //temporary cast for arithmetic sake
                let floor: isize = buff_height as isize - ceiling as isize;

                let mut shade;

                if distance_to_wall <= depth / 4.0 {
                    shade = '█';
                } else if distance_to_wall < depth / 3.0 {
                    shade = '▓';
                } else if distance_to_wall < depth / 2.0 {
                    shade = '▒';
                } else if distance_to_wall < depth {
                    shade = '░';
                } else {
                    shade = ' ';
                }

                for z in 0..buff_height {
                    if z as isize <= ceiling_con {
                        window_buffer[z * buff_width + i] = ' ' as u16;
                    } else if z as isize > ceiling_con && z <= floor as usize {
                        window_buffer[z * buff_width + i] = shade as u16;
                    } else {
                        // Shade floor based on distance
                        let floor_distance = 1.0 - ((z as f64 - buff_height as f64 / 2.0) / (buff_height as f64 / 2.0));
                        if floor_distance < 0.25 {
                            shade = '#';
                        } else if floor_distance < 0.5 {
                            shade = 'x';
                        } else if floor_distance < 0.75 {
                            shade = '.';
                        } else if floor_distance < 0.9 {
                            shade = '-';
                        } else {
                            shade = ' ';
                        }
                        window_buffer[z * buff_width + i] = shade as u16;
                    }
                }
            }

            winuser::wsprintfW(buff_sec_ptr, s_ptr);

            wincon::WriteConsoleOutputCharacterW(hconsole, buffer_ptr, 120 * 40, buff_coord, &mut dw_bytes_written);
        }
    }
}

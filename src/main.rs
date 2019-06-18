extern crate winapi;

use std::ptr;
use std::str;
use winapi::ctypes;
use winapi::shared::minwindef;
use winapi::shared::ntdef;
use winapi::um::{ wincon, winuser, winnt, wincontypes, winbase };
use std::ffi::CString;
use std::time::{ Instant };

fn main() {
    let buff_width = 120;
    let buff_height = 40;

    let player_x = 8.0;
    let player_y = 8.0;
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
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "#..............#";
    map += "################";

    let mut map_slice = &map.as_bytes(); //index-able reference slice

    let buff_coord = wincontypes::COORD {
        X: 0,
        Y: 0,
    };

    let mut window_buffer: Vec<ctypes::wchar_t> = vec!['*' as u16; buff_width * buff_height];
    let buffer_ptr = window_buffer.as_ptr();

    //ignore
    // let test = ["*".as_bytes(); 5 * 5];
    //
    // let utf8 = str::from_utf8(test[0]).unwrap();
    // let utf16: Vec<u16> = utf8.encode_utf16().collect();
    // let utf_string = String::from_utf16(&utf16).unwrap();
    // let utf_len = utf_string.len();
    //
    // let c_str = CString::new(utf_string).unwrap();
    // let ptr: *const u16 = c_str.as_ptr() as *const u16;

    // let dwBytesWritten: *mut u32 = 0 as *mut u32;

    let mut dw_bytes_written = 0;

    let mut time_point_1 = Instant::now();
    let mut time_point_2;

    unsafe {
        let hconsole = wincon::CreateConsoleScreenBuffer(winnt::GENERIC_READ | winnt::GENERIC_WRITE, 0, ptr::null(), wincon::CONSOLE_TEXTMODE_BUFFER, ntdef::NULL);

        wincon::SetConsoleActiveScreenBuffer(hconsole);


        loop {
            time_point_2 = Instant::now();
            let elapsed_time = time_point_2.duration_since(time_point_1);
            let in_nano = elapsed_time.as_micros() as f64 / 100_000.0;
            // let in_nano_con = in_nano / 100_000.0;
            time_point_1 = time_point_2;
            // println!("{:?}", in_nano);

            //Controls
            //Handle CCW Rotation
            let key_trigger_a = winuser::GetAsyncKeyState('A' as i32);

            let key_trigger_d = winuser::GetAsyncKeyState('D' as i32);

            if key_trigger_a != 0 && key_trigger_a == -32768 {
                player_a -= 0.1 * in_nano;
            }

            if key_trigger_d != 0 && key_trigger_d == -32768 {
                player_a += 0.1 * in_nano;
            }

            for i in 0..buff_width {
                //For each column, calculate the projected ray angle into world space
                let ray_angle = (player_a - player_fov / 2.0) + (i as f64 / buff_width as f64) * player_fov;

                let mut distance_to_wall = 0.0;
                let mut hit_wall = false;

                let eye_x = ray_angle.sin(); //Unit vector for ray in player space
                let eye_y = ray_angle.cos();

                while !hit_wall && distance_to_wall < depth {
                    distance_to_wall += 0.1;

                    let test_x = player_x + eye_x * distance_to_wall;
                    let test_y = player_y + eye_y * distance_to_wall;

                    let test_x_con = test_x as u32;
                    let test_y_con = test_y as u32;

                    //Test if ray is out of bounds
                    if test_x_con < 0 || test_x_con >= MAP_WIDTH || test_y_con < 0 || test_y_con >= MAP_HEIGHT {
                        hit_wall = true; //Just set distance to maximum depth
                        distance_to_wall = depth;
                    } else {
                        // Ray is inbounds so test to see if the ray cell is a wall block
                        let test_convert = test_y_con * MAP_WIDTH + test_x_con;
                        let test_con = test_convert as usize; //temporary cast for arithmetic sake
                        if map_slice[test_con] as char == '#' {
                            hit_wall = true;
                        }
                    }
                }

                //Calculate distance to ceiling and floor
                let ceiling = (buff_height as f64 / 2.0) - (buff_height as f64 / distance_to_wall);
                let ceiling_con = ceiling as usize; //temporary cast for arithmetic sake
                let floor = buff_height - ceiling_con;

                for z in 0..buff_height {
                    if z <= ceiling_con {
                        window_buffer[z * buff_width + i] = ' ' as u16;
                    } else if z > ceiling_con && z <= floor {
                        window_buffer[z * buff_width + i] = '#' as u16;
                    } else {
                        window_buffer[z * buff_width + i] = ' ' as u16;
                    }
                }
            }

            wincon::WriteConsoleOutputCharacterW(hconsole, buffer_ptr, 120 * 40, buff_coord, &mut dw_bytes_written);
        }
    }
}

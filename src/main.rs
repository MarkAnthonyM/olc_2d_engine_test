extern crate winapi;

use std::ptr;
use std::str;
use winapi::ctypes;
use winapi::shared::minwindef;
use winapi::shared::ntdef;
use winapi::um::{ wincon, winuser, winnt, wincontypes, winbase };
use std::ffi::CString;

fn main() {
    let buff_width = 120;
    let buff_height = 40;

    let player_x = 8.0;
    let player_y = 8.0;
    let player_a = 0.0;

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

    let mut map_slice = &map.as_bytes();

    let buff_coord = wincontypes::COORD {
        X: 0,
        Y: 0,
    };

    let mut window_buffer: Vec<ctypes::wchar_t> = vec!['#' as u16; buff_width * buff_height];
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

    unsafe {
        let hconsole = wincon::CreateConsoleScreenBuffer(winnt::GENERIC_READ | winnt::GENERIC_WRITE, 0, ptr::null(), wincon::CONSOLE_TEXTMODE_BUFFER, ntdef::NULL);

        wincon::SetConsoleActiveScreenBuffer(hconsole);

        loop {
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

                    //Test if ray is out of bounds
                    if test_x < 0.0 || test_x >= MAP_WIDTH.into() || test_y < 0.0 || test_y >= MAP_HEIGHT.into() {
                        hit_wall = true; //Just set distance to maximum depth
                        distance_to_wall = depth;
                    } else {
                        // Ray is inbounds so test to see if the ray cell is a wall block
                        let test_convert = test_y * MAP_WIDTH as f64 + test_x;
                        let test_con = test_convert as usize;
                        if map_slice[test_con] as char == '#' {
                            hit_wall = true;
                        }
                    }
                }

                //Calculate distance to ceiling and floor
                let ceiling = (buff_height as f64 / 2.0) - (buff_height as f64 / distance_to_wall);
                let floor = buff_height as f64 - ceiling;

                for z in 0..buff_height {
                    if z < ceiling as usize {
                        window_buffer[z * buff_height + i] = '.' as u16;
                    } else if z > ceiling as usize && z <= floor as usize {
                        window_buffer[z * buff_height + i] = '#' as u16;
                    } else {
                        window_buffer[z * buff_height + i] = ' ' as u16;
                    }
                }
            }

            wincon::WriteConsoleOutputCharacterW(hconsole, buffer_ptr, 120 * 40, buff_coord, &mut dw_bytes_written);
        }
    }
}

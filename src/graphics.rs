use crate::CPU;
use minifb::Window;
use minifb::WindowOptions;
use std::fs;
use std::vec;
pub fn create_image(cpu: &CPU) {
    let grid: [[u32; 256]; 224] = create_grid(cpu);

    let mut str: String = String::from("P1\n224 256\n");
    for y in 0..256 {
        for x in 0..224 {
            let pixel = grid[x][255 - y];
            str += &pixel.to_string();
        }
        str.push_str("\n");
    }

    let _ = fs::write("resources/img.pbm", str);
}
fn create_grid(cpu: &CPU) -> [[u32; 256]; 224] {
    let mut grid: [[u32; 256]; 224] = [[0; 256]; 224];
    for i in 0..224 as usize {
        for j in 0..32 as usize {
            let byte: u8 = cpu.memory[0x2400 + i * 32 + j];
            let array = byte_to_array(byte);
            array.iter().enumerate().for_each(|(index, f)| {
                if *f == 1 {
                    grid[i][j * 8 + index] = 0x00FFFFFF;
                }
            });
        }
    }
    grid
}
fn byte_to_array(byte: u8) -> [u8; 8] {
    let array2: [u8; 8] = core::array::from_fn(|i| (byte >> i) & 0x01);
    array2
}
pub fn open_window() -> Option<Window> {
    let window = match Window::new("I love CP", 224, 256, WindowOptions{resize:true,..WindowOptions::default()}) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return None;
        }
    };
    println!("hi lol");
    Some(window)
}
pub fn update_window(cpu: &CPU, window: &mut Window) {
    let grid = create_grid(cpu);
    let mut buff: Vec<u32> = Vec::new();
    for y in 0..256 {
        for x in 0..224 {
            let pixel = grid[x][255 - y];
            buff.push(pixel as u32);
        }
    }
    window.update_with_buffer(&buff, 224, 256).unwrap();
}

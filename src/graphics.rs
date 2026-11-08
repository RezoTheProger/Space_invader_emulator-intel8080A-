use crate::CPU;
pub fn create_image(cpu: &CPU) {
    let mut grid: [[u8; 256]; 224] = [[0; 256]; 224];
    for i in 0..224 as usize {
        for j in 0..32 as usize {
            let byte: u8 = cpu.memory[0x2400 + i * 32 + j];
            let array = byte_to_array(byte);
            array.iter().enumerate().for_each(|(index, f)| {
                if *f == 1 {
                    grid[i][j * 8 + index] = 1;
                }
            });
        }
    }
}
fn byte_to_array(byte: u8) -> [u8; 8] {
    let array2: [u8; 8] = core::array::from_fn(|i| (byte >> i) & 0x01);
    array2
}

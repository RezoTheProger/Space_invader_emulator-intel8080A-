mod cpu;
mod graphics;
use cpu::CPU;
use graphics::create_image;
use graphics::open_window;
use std::{
    fs, thread,
    time::{Duration, Instant},
};

use crate::graphics::update_window;
fn main() {
    println!("Hello, world!");
    let mut cpu: CPU = CPU::new();

    let h_file: Vec<u8> = fs::read("resources/invaders.h").unwrap();
    let g_file: Vec<u8> = fs::read("resources/invaders.g").unwrap();
    let f_file: Vec<u8> = fs::read("resources/invaders.f").unwrap();
    let e_file: Vec<u8> = fs::read("resources/invaders.e").unwrap();

    cpu.load_rom(&h_file, 0x0000);
    cpu.load_rom(&g_file, 0x0800);
    cpu.load_rom(&f_file, 0x1000);
    cpu.load_rom(&e_file, 0x1800);
    let mut window = open_window().unwrap();
    create_image(&cpu);
    loop {
        for _ in 0..60 {
            let instant = Instant::now();
            for _ in 0..3_500 {
                step(&mut cpu);
            }

            cpu.interrupt(1);
            for _ in 0..3_500 {
                step(&mut cpu);
            }
            cpu.interrupt(2);
            update_window(&cpu, &mut window);
            let elapsed = instant.elapsed();
            let budget = Duration::from_micros(16_667);
            match budget.checked_sub(elapsed) {
                Some(n) => thread::sleep(n),
                None => continue,
            }
        }
    }
}
fn step(cpu: &mut CPU) {
    if cpu.halted {
        println!("Cpu is halted");
        return;
    }

    let opcode = cpu.fetch();
    cpu.decode(opcode);
}

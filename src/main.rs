mod cpu;
mod graphics;
use cpu::CPU;
use graphics::create_image;
use std::fs;
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

    for _ in 0..60 {
        for _ in 0..3_500 {
            step(&mut cpu);
        }

        cpu.interrupt(1);
        for _ in 0..3_500 {
            step(&mut cpu);
        }
        cpu.interrupt(2);
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

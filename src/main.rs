mod cpu;
mod graphics;
use cpu::CPU;
use graphics::create_image;
use graphics::open_window;
use minifb::Key;
use minifb::Window;
use std::{
    fs, thread,
    time::{Duration, Instant},
};

use crate::graphics::update_window;
fn main() {
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
            cpu.ports[0] = 0x0E;
            cpu.ports[1] = 0x08;
            cpu.ports[2] = 0x00;
            key_checker(&mut cpu, &window);

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
fn key_checker(cpu: &mut CPU, window: &Window) {
    if window.is_key_down(Key::F) {
        cpu.ports[0] |= 0x1F;
    }
    if window.is_key_down(Key::Left) {
        cpu.ports[0] |= 0x2F;
    }
    if window.is_key_down(Key::Right) {
        cpu.ports[0] |= 0x4F;
    }
    //-----
    if window.is_key_down(Key::C) {
        cpu.ports[1] |= 0x01;
    }
    if window.is_key_down(Key::NumPad2) {
        cpu.ports[1] |= 0x02;
    }
    if window.is_key_down(Key::NumPad1) {
        cpu.ports[1] |= 0x04;
    }
    if window.is_key_down(Key::F) {
        cpu.ports[1] |= 0x18;
    }
    if window.is_key_down(Key::Left) {
        cpu.ports[1] |= 0x28;
    }
    if window.is_key_down(Key::Right) {
        cpu.ports[1] |= 0x48;
    }
    //-----
    if window.is_key_down(Key::F) {
        cpu.ports[2] |= 0x10;
    }
    if window.is_key_down(Key::Left) {
        cpu.ports[2] |= 0x20;
    }
    if window.is_key_down(Key::Right) {
        cpu.ports[2] |= 0x40;
    }
}

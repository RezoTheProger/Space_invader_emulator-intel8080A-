mod cpu;
use cpu::CPU;
use std::fs;

fn main() {
    println!("Hello, world!");
    let mut cpu: CPU = CPU::new();
    /*
    let mut h_file: Vec<u8> = fs::read("resources/invaders.h").unwrap();
    let mut g_file: Vec<u8> = fs::read("resources/invaders.g").unwrap();
    let mut f_file: Vec<u8> = fs::read("resources/invaders.f").unwrap();
    let mut e_file: Vec<u8> = fs::read("resources/invaders.e").unwrap();
    */
    let mut test: Vec<u8> = fs::read("resources/TST8080.COM").unwrap();
    tester(&mut cpu, &mut test);

    test = fs::read("resources/8080PRE.COM").unwrap();

    cpu = CPU::new();
    tester(&mut cpu, &mut test);
    test = fs::read("resources/8080EXM.COM").unwrap();
    cpu = CPU::new();
    tester(&mut cpu, &mut test);
    test = fs::read("resources/8080EXER.COM").unwrap();
    cpu = CPU::new();
    tester(&mut cpu, &mut test);
    test = fs::read("resources/CPUTEST.COM").unwrap();
    cpu = CPU::new();
    tester(&mut cpu, &mut test);
}
fn tester(cpu: &mut CPU, test: &mut Vec<u8>) {
    let mut rom_data: Vec<u8> = Vec::new();
    rom_data.append(test);
    cpu.load_rom(&rom_data);
    let mut opcode: u8 = cpu.memory[cpu.pc as usize];
    loop {
        if cpu.halted {
            println!("Cpu is halted");
            break;
        }
        if cpu.pc == 0x0000 {
            break;
        }
        if cpu.pc == 0x0005 {
            cpu.bdos();
            continue;
        }

        opcode = cpu.fetch();
        cpu.decode(opcode);
    }
}

use bus::Bus;
use cpu;
use cpu::Cpu;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            bus: Bus::new(), 
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus.ram_write_byte(cpu::PROGRAM_START + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
        println!("Cpu state: {:?}", self.cpu);
    }
}

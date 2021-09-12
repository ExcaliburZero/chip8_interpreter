use chip8_interpreter::cpu;

fn main() {
    let mut cpu = cpu::CPU::default();
    println!("Created CPU representation");

    cpu.load_default_font();
    println!("Loaded default font");
}

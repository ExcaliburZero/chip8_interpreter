extern crate clap;

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

use clap::{App, Arg, ArgMatches};

use chip8_interpreter::cpu;

fn main() {
    let matches = App::new("chip8_interpreter")
        .version("0.1.0")
        .author("Christopher Wells")
        .about("")
        .arg(Arg::with_name("ROM").required(true).index(1))
        .get_matches();

    run(&matches).unwrap();
}

fn run(args: &ArgMatches) -> Result<(), String> {
    let mut cpu = cpu::CPU::default();
    println!("Created CPU representation");

    cpu.load_default_font()?;
    println!("Loaded default font");

    let rom_filepath = args
        .value_of("ROM")
        .ok_or("User did not provide ROM argument")?;
    load_rom(&mut cpu, rom_filepath)?;
    println!("Loaded ROM: {}", rom_filepath);

    cpu.initialize_program_counter();
    println!("Initialized program counter");

    Ok(())
}

fn load_rom(cpu: &mut cpu::CPU, filepath: &str) -> Result<(), String> {
    let rom = un_io_result(load_file_bytes(filepath))?;

    cpu.load_rom(&rom)
}

fn un_io_result<R>(result: io::Result<R>) -> Result<R, String> {
    match result {
        Ok(r) => Ok(r),
        Err(msg) => Err(msg.to_string()),
    }
}

fn load_file_bytes(filepath: &str) -> io::Result<Vec<u8>> {
    let f = File::open(filepath)?;
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

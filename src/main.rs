extern crate clap;

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::{thread, time};

use clap::{App, Arg, ArgMatches};
use minifb::{Key, Window, WindowOptions};

use chip8_interpreter::views::View;
use chip8_interpreter::{cpu, screen, views};

const MAX_INSTRUCTIONS_PER_SECOND: u64 = 700;
const ONE_SECOND_IN_MICROSECONDS: u64 = 1000000;

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

    let stdout = io::stdout();
    //let mut view = views::CliView::new(stdout.lock());
    let mut view = views::MiniFbView::new(
        "CHIP-8".to_string(),
        64 * 2,
        32 * 2,
        WindowOptions::default(),
    );
    println!("Created view");

    println!("Starting execution");

    view.open(&cpu.screen);

    loop {
        let inputs = view.get_inputs()?;
        let screen_changed = cpu.step(&time::Instant::now(), &inputs)?;

        if screen_changed == cpu::ScreenChanged::Changed {
            // Clear the screen so we can redraw it
            print!("\x1B[32A"); // Move the cursor back to the start of the screen
            print!("\x1B[J"); // Clear everything below the cursor
                              //println!("----------------------");

            // Redraw the screen
            if view.update(&cpu.screen) == views::ViewState::Closed {
                break;
            }

            //let sleep_constant = time::Duration::from_millis(40);
            //thread::sleep(sleep_constant);
        }

        let sleep_constant =
            time::Duration::from_micros(ONE_SECOND_IN_MICROSECONDS / MAX_INSTRUCTIONS_PER_SECOND);
        thread::sleep(sleep_constant);
    }

    view.close();

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

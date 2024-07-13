use std::{env, io::{self, BufRead, Write}};

use spin_freeze::{frames_to_freeze, freeze_comands, DELTA_TIME};

fn get_user_input(prompt: &str)->io::Result<String>{
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().lock().lines().next().unwrap()
}
fn main()->io::Result<()>{
    let mut args = env::args();
    args.next();
    let arg = args.next();
    let cycle = match arg{
        None=>{println!("You must enter \"wait\" or \"cycle\" to choose a mode. Wait mode is for just waiting for freeze, cycle mode is for waiting with a repeat");
            std::process::exit(0)
        },
        Some(s)=>{
            s.to_lowercase()=="cycle"
        }
    };
    if cycle {println!("Enter configurations for cycle mode")} else {println!("Enter configurations for wait mode")};
    let time_active: f32 = get_user_input("Time Active when commands should be: ")?.parse().unwrap();
    let chapter_time: usize = get_user_input("Chapter time in frames when commands should be: ")?.parse().unwrap();
    let frames_before_freeze: usize = get_user_input("Number of frames before freeze should occur from commands: ")?.parse().unwrap();
    if cycle{
        let cycle_length: usize = get_user_input("Cycle length in frames: ")?.parse().unwrap();
        let c_info = spin_freeze::get_cycle_wait_info(time_active, cycle_length, chapter_time, frames_before_freeze);
        println!("The wait will take {} cycles", c_info.cycle_count);
        println!("Leaving {} frames remaining", c_info.remaining_frames);
        let before = get_user_input("Will the remaining frames be placed before or after the read commands? ")?;
        let before = before.to_lowercase().contains('b');
        if before {
            let mut time_active = time_active;
            let mut chapter_time = chapter_time;
            for _ in 0..c_info.remaining_frames{
                time_active+=DELTA_TIME;
                chapter_time+=1;
            }
            println!("{}",spin_freeze::cycle_commands(time_active, cycle_length, chapter_time, frames_before_freeze).0);
        } else {
            println!("{}",spin_freeze::cycle_commands(time_active, cycle_length, chapter_time, frames_before_freeze).0);
        }
    } else {
        let wait_time = frames_to_freeze(time_active)-frames_before_freeze;
        println!("The freeze wait will take {} frames",wait_time);
        println!("{}",freeze_comands(time_active, chapter_time, frames_before_freeze));
    }
    Ok(())
}
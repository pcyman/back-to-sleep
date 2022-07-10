use chrono::prelude::Timelike;
use std::process::Command;
use clap::Parser;

/// Detect if cursor has moved and make the computer go back to sleep
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Check every x seconds
    #[clap(short, long, value_parser, default_value_t = 60)]
    count: u64,
}

fn get_current_mouse_location() -> (usize, usize) {
    let output = Command::new("xdotool")
        .arg("getmouselocation")
        .output()
        .expect("Failed to execute xdotool");
    let mouse_location_output: String = String::from_utf8(output.stdout).unwrap();
    let words: Vec<&str> = mouse_location_output.split(" ").collect();
    let x: String = String::from(words[0].split(":").collect::<Vec<&str>>()[1]);
    let y: String = String::from(words[1].split(":").collect::<Vec<&str>>()[1]);

    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

fn go_to_sleep() {
    let time = chrono::offset::Local::now().time().format("%H:%M");
    println!("[{}] Going back to sleep!", time);
    Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Failed to execute xdotool");
}

fn main() {
    let args = Args::parse();
    let count = args.count;

    let (initial_x, initial_y) = get_current_mouse_location();
    let (mut x, mut y);
    loop {
        go_to_sleep();
        std::thread::sleep(std::time::Duration::from_secs(count));
        (x, y) = get_current_mouse_location();
        if x != initial_x && y != initial_y {
            break
        }
    }

    println!(
        "Mouse location changed from ({}, {}) to ({}, {}) - exiting.",
        initial_x,
        initial_y,
        x,
        y
    );
}

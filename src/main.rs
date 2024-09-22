use clap::Parser;
use std::process::Command;

/// Detect if cursor has moved and make the computer go back to sleep
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Check every x seconds
    #[clap(short, long, value_parser, default_value_t = 60)]
    count: u64,
}

fn go_to_sleep() {
    let time = chrono::offset::Local::now().time().format("%H:%M");
    println!("[{}] Going back to sleep!", time);
    Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Failed to execute systemctl");
}

enum Environment {
    X11,
    Hyprland,
}

impl Environment {
    fn get_cursor_location(&self) -> (usize, usize) {
        match self {
            Environment::X11 => || -> (usize, usize) {
                let output = Command::new("xdotool")
                    .arg("getmouselocation")
                    .output()
                    .expect("Failed to execute xdotool");
                let mouse_location_output: String = String::from_utf8(output.stdout).unwrap();
                let words: Vec<&str> = mouse_location_output.split(" ").collect();
                let x: String = String::from(words[0].split(":").collect::<Vec<&str>>()[1]);
                let y: String = String::from(words[1].split(":").collect::<Vec<&str>>()[1]);

                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            }(),
            Environment::Hyprland => || -> (usize, usize) {
                let output = Command::new("hyprctl")
                    .arg("cursorpos")
                    .output()
                    .expect("Failed to execute hyprctl");
                let mouse_location_output: String = String::from_utf8(output.stdout).unwrap();
                let words: Vec<&str> = mouse_location_output
                    .split(", ")
                    .map(|word| word.trim())
                    .collect();
                (
                    words[0].parse::<usize>().unwrap(),
                    words[1].parse::<usize>().unwrap(),
                )
            }(),
        }
    }
}

fn get_current_environment() -> Result<Environment, String> {
    let xdg_session_type = std::env::var("XDG_SESSION_TYPE");
    let desktop_session = std::env::var("DESKTOP_SESSION");

    match (xdg_session_type, desktop_session) {
        (Ok(xdg_session_type), _) if xdg_session_type == "x11" => Ok(Environment::X11),
        (_, Ok(desktop_session)) if desktop_session == "hyprland" => Ok(Environment::Hyprland),
        _ => Err("Unsupported environment".to_owned()),
    }
}

fn main() {
    let args = Args::parse();
    let count = args.count;

    let environment = get_current_environment().unwrap_or_else(|err| {
        eprintln!("Error getting the current environment: {}", err);
        std::process::exit(1);
    });

    let (initial_x, initial_y) = environment.get_cursor_location();
    let (mut x, mut y);
    loop {
        go_to_sleep();
        std::thread::sleep(std::time::Duration::from_secs(count));
        (x, y) = environment.get_cursor_location();
        if x != initial_x && y != initial_y {
            break;
        }
    }

    println!(
        "Mouse location changed from ({}, {}) to ({}, {}) - exiting.",
        initial_x, initial_y, x, y
    );
}

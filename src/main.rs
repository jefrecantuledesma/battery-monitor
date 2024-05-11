use std::{fs, process::Command};

fn main() {
    let battery_path = "/sys/class/power_supply/BAT1/capacity";
    match fs::read_to_string(battery_path) {
        Ok(contents) => {
            let percentage: i32 = contents.trim().parse().unwrap();
            if percentage <= 3 {
                println!("Battery is at {}%, hibernating...", percentage);
                Command::new("systemctl")
                    .args(&["hibernate"])
                    .status()
                    .expect("Failed to execute command");
            }
        }
        Err(e) => println!("Error reading battery capacity: {}", e),
    }
}

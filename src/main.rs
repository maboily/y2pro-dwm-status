extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::f32;


fn main() {
	while (true) {
		let charge_pct = get_battery_charge_pct();
		let is_charging = is_battery_charging();
		let power_usage = get_battery_power_usage();

		let time = time::now();


		let charge_txt = if is_charging { "+" } else { "-" };
		let time_str = time.strftime("%Y-%m-%d %H:%M").unwrap();
		let power_usage_watt = get_power_usage_watt(power_usage);

		let status_text = format!(
			"{} [{}{}%, Load: {:04.1}W]",
			time_str,
			charge_txt,
			charge_pct,
			power_usage_watt
		);

		run_set_root(status_text);

		sleep(Duration::from_millis(500));
	}
}

fn run_set_root(status_text: String) {
	Command::new("xsetroot")
			.arg("-name")
			.arg(status_text)
			.output()
			.expect("Failed to xsetroot");
}

fn get_power_usage_watt(power_usage_microwatt: i32) -> f32 {
	let power_usage_microwatt_float = power_usage_microwatt as f32;

	power_usage_microwatt_float * 0.000001
}

fn get_battery_power_usage() -> i32 {
	let power_usage_path = "/sys/class/power_supply/BAT1/power_now".to_string();
	let power_usage_str = read_file_to_str(power_usage_path);
	let power_usage_trimmed = power_usage_str.trim();

	power_usage_trimmed.parse::<i32>().unwrap()
}

fn get_battery_charge_pct() -> i8 {
	let charge_pct_path = "/sys/class/power_supply/BAT1/capacity".to_string();
	let charge_pct_str = read_file_to_str(charge_pct_path);
	let charge_pct_trimmed = charge_pct_str.trim();

	charge_pct_trimmed.parse::<i8>().unwrap()
}

fn is_battery_charging() -> bool {
	let charging_path = "/sys/class/power_supply/BAT1/status".to_string();
	let charging_file_content = read_file_to_str(charging_path);

	!charging_file_content.contains("Discharging")
}

fn read_file_to_str(path: String) -> String {
	let mut file = match File::open(path) {
		Err(why) => panic!("couldn't open file: {}", why),
		Ok(file) => file
	};
	let mut contents = String::new();
	file.read_to_string(&mut contents);

	contents
}
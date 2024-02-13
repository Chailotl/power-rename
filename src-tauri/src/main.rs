// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aho_corasick::AhoCorasick;
use chrono::{Datelike, Local, Timelike};
use regex::RegexBuilder;

const PATTERNS: [&str; 17] = [
	"$YYYY", "$YY", "$Y", "$MMMM", "$MMM", "$MM", "$M", "$DDDD", "$DDD", "$DD", "$D", "$hh", "$h",
	"$mm", "$m", "$ss", "$s",
];

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn rename(
	mut search: String,
	mut replace: String,
	use_regex: bool,
	match_all: bool,
	case_sensitive: bool,
) {
	// Build regex
	if !use_regex {
		search = regex::escape(&search);
	}

	let re = RegexBuilder::new(&search)
		.case_insensitive(!case_sensitive)
		.build()
		.unwrap();

	// TODO: move to a singleton
	let ac = AhoCorasick::builder()
		.match_kind(aho_corasick::MatchKind::LeftmostFirst)
		.build(PATTERNS)
		.unwrap();

	// Replace substrings in replace string
	let replace_with = {
		let mut list = vec![];

		let now = Local::now();

		let year = now.year();
		list.push(year.to_string());
		list.push(format!("{:0>2}", year % 100));
		list.push((year % 10).to_string());

		let month = now.month().to_string();
		list.push(now.format("%B").to_string());
		list.push(now.format("%b").to_string());
		list.push(format!("{:0>2}", month));
		list.push(month);

		let day = now.day().to_string();
		list.push(now.format("%A").to_string());
		list.push(now.format("%a").to_string());
		list.push(format!("{:0>2}", day));
		list.push(day);

		let hour = now.hour().to_string();
		list.push(format!("{:0>2}", hour));
		list.push(hour);

		let minute = now.minute().to_string();
		list.push(format!("{:0>2}", minute));
		list.push(minute);

		let second = now.second().to_string();
		list.push(format!("{:0>2}", second));
		list.push(second);

		list
	};

	replace = ac.replace_all(&replace, &replace_with);

	// TODO: operate on filenames
	let mut test = "Hello [ol] world!".to_string();

	test = re
		.replacen(&test, if match_all { 0 } else { 1 }, replace)
		.to_string();

	println!("{}", test);
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![rename])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

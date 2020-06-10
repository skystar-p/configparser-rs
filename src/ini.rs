//!ini
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

pub fn load(path: &str) -> HashMap<String, HashMap<String, String>> {
	let path = Path::new(path);
	let display = path.display();

	let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file,
	};

	let mut s = String::new();
	let mut section = "DEFAULT";
	match file.read_to_string(&mut s) {
		Err(why) => panic!("couldn't read {}: {}", display, why),
		Ok(_) => for lines in s.lines() {
			let trimmed = lines.trim();
			match trimmed.find('[') {
				Some(start) => match trimmed.rfind(']') {
					Some(end) => {
						section = &trimmed[start+1..end];
					},
					None => panic!("Found opening bracket at {} but no closing bracket", start)
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(section) {
							Some(valmap) => {
								valmap.insert(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string());
							}
							None => {
								let valmap: HashMap<String, String> =
								[(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string())]
								.iter().cloned().collect();
								map.insert(section.to_string(), valmap);
							}
						}
					}
					None => ()
				}
			}
		}
	}
	return map;
}

pub struct Ini {
	map: HashMap<String, HashMap<String, String>>
}

impl Ini {

	pub fn new() -> Result<Ini, Box<dyn Error>> {
		let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
		let mut inimap = Ini {
			map
		}
	}

	pub fn load(&mut self, path: &str) -> Result<(), String> {
		let path = Path::new(path);
		let display = path.display();

		let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

		let mut file = match File::open(&path) {
			Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
			Ok(file) => file
		};

		let mut s = String::new();
		self.map = match file.read_to_string(&mut s) {
			Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
			Ok(_) => match self.parse(s) {
				Err(why) => return Err(why),
				Ok(map) => map
			}
		};
		Ok(())
	}

	fn parse(&self, input: String) -> Result <HashMap<String, HashMap<String, String>>, String> {
		let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
		let mut section = "DEFAULT";
		for lines in input.lines() {
			let trimmed = lines.trim();
			match trimmed.find('[') {
				Some(start) => match trimmed.rfind(']') {
					Some(end) => {
						section = &trimmed[start+1..end];
					},
					None => return Err(format!("Found opening bracket at {} but no closing bracket", start))
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(section) {
							Some(valmap) => {
								valmap.insert(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string());
							}
							None => {
								let valmap: HashMap<String, String> =
								[(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string())]
								.iter().cloned().collect();
								map.insert(section.to_string(), valmap);
							}
						}
					}
					None => ()
				}
			}
		}
		Ok(map)
	}
}
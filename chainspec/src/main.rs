// Copyright 2015-2020 Vast Technologies (UK) Ltd.
// This file is part of Vast.

// Vast is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Vast is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Vast.  If not, see <http://www.gnu.org/licenses/>.

extern crate serde_json;
extern crate vastjson;

use std::{fs, env, process};
use vastjson::spec::Spec;

fn quit(s: &str) -> ! {
	println!("{}", s);
	process::exit(1);
}

fn main() {
	let mut args = env::args();
	if args.len() != 2 {
		quit("You need to specify chainspec.json\n\
		\n\
		./chainspec <chainspec.json>");
	}

	let path = args.nth(1).expect("args.len() == 2; qed");
	let file = match fs::File::open(&path) {
		Ok(file) => file,
		Err(_) => quit(&format!("{} could not be opened", path)),
	};

	let spec: Result<Spec, _> = serde_json::from_reader(file);

	if let Err(err) = spec {
		quit(&format!("{} {}", path, err.to_string()));
	}

	println!("{} is valid", path);
}

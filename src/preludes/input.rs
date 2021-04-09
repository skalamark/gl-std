// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::io::Write;

use gl_core::preludes::*;

pub fn input(args: Vec<Object>) -> Result<Object, Exception> {
	if args.len() > 0 {
		print!("{}", match &args[0] {
			Object::String(s) => s.clone(),
			o => format!("{}", o),
		});

		match std::io::stdout().flush() {
			Ok(_) => Ok(Object::Null),
			Err(err) => Err(Exception::in_runtime(Except::error(format!("{}", err)))),
		}?;
	}

	let mut line = String::new();
	match std::io::stdin().read_line(&mut line) {
		Ok(_) => Ok(Object::String(line)),
		Err(err) => Err(Exception::in_runtime(Except::error(format!("{}", err)))),
	}
}

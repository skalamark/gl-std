// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::io::Write;

use gl_core::preludes::*;

pub fn print(args: Vec<Object>) -> Result<Object, Exception> {
	let fmt_string: String = format_print(args);

	print!("{}", fmt_string);
	match std::io::stdout().flush() {
		Ok(_) => Ok(Object::Null),
		Err(err) => Err(Exception::in_runtime(Except::error(format!("{}", err)))),
	}
}

pub fn println(args: Vec<Object>) -> Result<Object, Exception> {
	let fmt_string: String = format_print(args);

	println!("{}", fmt_string);
	Ok(Object::Null)
}

fn format_print(args: Vec<Object>) -> String {
	let mut fmt_string: String = String::new();

	for (i, object) in args.iter().enumerate() {
		match object {
			Object::String(s) => fmt_string.push_str(&s),
			o => fmt_string.push_str(&format!("{}", o)),
		}

		if i < args.len() - 1 {
			fmt_string.push(' ');
		}
	}

	fmt_string
}

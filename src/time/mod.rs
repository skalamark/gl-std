// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::{thread, time};

use gl_core::preludes::*;

pub fn sleep(args: Vec<Object>) -> Result<Object, Exception> {
	if let Object::Integer(duration_integer) = args.get(0).unwrap() {
		let duration: time::Duration =
			time::Duration::from_millis(duration_integer.to_u64().unwrap());
		thread::sleep(duration);
	} else {
		let exception: Exception = Exception::in_runtime(Except::type_(format!(
			"an integer is required (got type {})",
			args.get(0).unwrap().typer()
		)));
		return Err(exception);
	};

	Ok(Object::Null)
}

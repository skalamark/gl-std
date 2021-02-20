// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use firmata;
use gl_core::preludes::*;
use std::thread;
use std::time;

pub fn sleep(
	args: Vec<Arc<Mutex<Box<dyn Object>>>>, module: String, position: Position,
) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
	let a = &args[0];
	if let Some(duration_integer) = a.lock().unwrap().downcast_ref::<Integer>() {
		let duration: time::Duration =
			time::Duration::from_millis(duration_integer.value.to_string().parse().unwrap());
		thread::sleep(duration);
	} else {
		let mut exception: Exception = Exception::new(
			Except::type_(format!(
				"an integer is required (got type {})",
				a.lock().unwrap().typer()
			)),
			true,
		);
		exception.push(ExceptionPoint::new(module, position));
		return Err(exception);
	};

	Ok(Arc::new(Mutex::new(Box::new(Null::new()))))
}

pub fn arduino(
	_: Vec<Arc<Mutex<Box<dyn Object>>>>, _: String, _: Position,
) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
	Ok(Arc::new(Mutex::new(Box::new(Arduino::new()))))
}

pub struct Arduino {
	pub board: firmata::Board,
	pub env: HashMap<
		String,
		fn(
			Vec<Arc<Mutex<Box<dyn Object>>>>,
			String,
			Position,
		) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception>,
	>,
}

impl Object for Arduino {
	fn getattribute(&mut self, name: String) -> Option<Box<dyn Object>> {
		if let Some(object) = self.env.get(&name) {
			Some(Box::new(FunctionRust::new(object.clone())))
		} else {
			None
		}
	}

	fn call(
		&mut self, attr: String, args: Vec<Arc<Mutex<Box<dyn Object>>>>, module: String,
		position: Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
		if attr == "on" {
			self.on(args, module, position)
		} else if attr == "off" {
			self.off(args, module, position)
		} else {
			return Object::call(self, attr, args, module, position);
		}
	}
}

impl std::fmt::Display for Arduino {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "nulls")
	}
}

impl std::fmt::Debug for Arduino {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "nulls")
	}
}

impl Arduino {
	pub fn new() -> Self {
		let env: HashMap<
			String,
			fn(
				Vec<Arc<Mutex<Box<dyn Object>>>>,
				String,
				Position,
			) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception>,
		> = HashMap::new();

		Self {
			board: firmata::Board::new("/dev/ttyUSB0"),
			env,
		}
	}

	pub fn on(
		&mut self, _: Vec<Arc<Mutex<Box<dyn Object>>>>, _: String, _: Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
		self.board.digital_write(13, 1);
		Ok(Arc::new(Mutex::new(Box::new(Null::new()))))
	}

	pub fn off(
		&mut self, _: Vec<Arc<Mutex<Box<dyn Object>>>>, _: String, _: Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
		self.board.digital_write(13, 0);
		Ok(Arc::new(Mutex::new(Box::new(Null::new()))))
	}
}

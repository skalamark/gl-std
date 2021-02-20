// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
// use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent, Modifiers};

pub fn println_builtin(
	args: Vec<Arc<Mutex<Box<dyn Object>>>>, _: String, _: Position,
) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
	let mut fmt_string: String = String::new();
	for (i, object) in args.iter().enumerate() {
		fmt_string.push_str(&format!("{}", object.lock().unwrap()));
		if i < args.len() - 1 {
			fmt_string.push_str(" ");
		}
	}
	println!("{}", fmt_string);
	Ok(Arc::new(Mutex::new(Box::new(Null::new()))))
}

// pub fn len_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, module: String, position: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	Ok(match &args[0] {
// 		String::new(string) => Integer::new(Integer::new(BigInt::from(string.len()))),
// 		Vec::new(array) => Integer::new(Integer::new(BigInt::from(array.len()))),
// 		HashMap::new(hashmap) => Integer::new(Integer::new(BigInt::from(hashmap.len()))),
// 		object => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(
// 				ExcptErr::invalid_syntax(format!(
// 					"object of type '{}' has no len()",
// 					object.typer()
// 				)),
// 				true,
// 			);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 	})
// }

// pub fn push_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, module: String, position: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	let value: Obj = args[1].copy();

// 	Ok(match &args[0] {
// 		Vec::new(array) => {
// 			let mut r = vec![];
// 			for object in array.iter() {
// 				r.push(object.copy());
// 			}
// 			r.push(value);
// 			Vec::new(r)
// 		}
// 		object => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(
// 				ExcptErr::invalid_syntax(format!(
// 					"object of type '{}' has no push()",
// 					object.typer()
// 				)),
// 				true,
// 			);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 	})
// }

// pub fn insert_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, module: String, position: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	let key: Obj = args[1].copy();
// 	let value: Obj = args[2].copy();

// 	Ok(match &args[0] {
// 		HashMap::new(hashmap) => {
// 			let mut r = HashMap::new();
// 			for (key, value) in hashmap.iter() {
// 				r.insert(key.copy(), value.copy());
// 			}
// 			r.insert(key, value);
// 			HashMap::new(r)
// 		}
// 		object => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(
// 				ExcptErr::invalid_syntax(format!(
// 					"object of type '{}' has no insert()",
// 					object.typer()
// 				)),
// 				true,
// 			);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 	})
// }

// pub fn input_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, module: String, position: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	let prompt: String = match &args[0] {
// 		String::new(string) => format!("{}", string),
// 		_ => format!("{}", &args[0]),
// 	};

// 	let mut editor: Editor<()> = Editor::<()>::new();
// 	editor.bind_sequence(
// 		KeyEvent::new('\t', Modifiers::NONE),
// 		Cmd::Insert(1, format!("\t")),
// 	);

// 	match editor.readline(&prompt) {
// 		Ok(buffer) => return Ok(String::new(buffer)),
// 		Err(ReadlineError::Interrupted) => {
// 			let mut exception: ExceptionMain =
// 				ExceptionMain::new(ExcptErr::keyboard_interrupt(format!("")), true);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 		Err(ReadlineError::Eof) => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(ExcptErr::eof(format!("")), true);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 		Err(_) => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(ExcptErr::eof(format!("")), true);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 	}
// }

// pub fn integer_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, module: String, position: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	Ok(match &args[0] {
// 		Integer::new(integer) => Integer::new(Integer::new(integer.integer.clone())),
// 		Boolean::new(boolean) => Integer::new(Integer::new(
// 			BigInt::parse_bytes(if boolean.boolean { b"1" } else { b"0" }, 10).unwrap(),
// 		)),
// 		String::new(string) => Integer::new(Integer::new(
// 			BigInt::parse_bytes(string.as_bytes(), 10).unwrap(),
// 		)),
// 		object => {
// 			let mut exception: ExceptionMain = ExceptionMain::new(
// 				ExcptErr::invalid_syntax(format!(
// 					"integer() argument must be a string or a number, not '{}'",
// 					object.typer()
// 				)),
// 				true,
// 			);
// 			exception.push(Excpt::new(module, position));
// 			return Err(exception);
// 		}
// 	})
// }

// pub fn string_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, _: String, _: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	Ok(match &args[0] {
// 		object => String::new(object.to_string()),
// 	})
// }

// pub fn boolean_builtin(
// 	args: Vec<Mutex<Box<dyn Object>>>, _: String, _: Position,
// ) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
// 	Ok(match &args[0] {
// 		Null::new => Boolean::new(Boolean::new(false)),
// 		Boolean::new(boolean) if boolean.boolean == false => Boolean::new(Boolean::new(false)),
// 		_ => Boolean::new(Boolean::new(true)),
// 	})
// }

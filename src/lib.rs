// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;
use time::{arduino, sleep};
mod builtins;
mod time;

pub struct Std {}

impl Std {
	pub fn new() -> Env {
		let mut env: Env = Env::new();
		Self::builtins(&mut env);
		Self::time(&mut env);
		env
	}

	fn builtins(env: &mut Env) {
		env.set(
			&format!("println"),
			Arc::new(Mutex::new(Box::new(FunctionRust::new(
				builtins::println_builtin,
			)))),
		);
		// env.set(
		// 	&format!("println"),
		// 	Object::BuiltinRust {
		// 		name: format!("println"),
		// 		len_args: -1,
		// 		builtinfn: builtins::println_builtin,
		// 	},
		// );
	}
	// 	env.set(
	// 		&format!("len"),
	// 		Object::BuiltinRust {
	// 			name: format!("len"),
	// 			len_args: 1,
	// 			builtinfn: builtins::len_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("push"),
	// 		Object::BuiltinRust {
	// 			name: format!("push"),
	// 			len_args: 2,
	// 			builtinfn: builtins::push_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("insert"),
	// 		Object::BuiltinRust {
	// 			name: format!("insert"),
	// 			len_args: 3,
	// 			builtinfn: builtins::insert_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("input"),
	// 		Object::BuiltinRust {
	// 			name: format!("input"),
	// 			len_args: 1,
	// 			builtinfn: builtins::input_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("integer"),
	// 		Object::BuiltinRust {
	// 			name: format!("integer"),
	// 			len_args: 1,
	// 			builtinfn: builtins::integer_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("string"),
	// 		Object::BuiltinRust {
	// 			name: format!("string"),
	// 			len_args: 1,
	// 			builtinfn: builtins::string_builtin,
	// 		},
	// 	);
	// 	env.set(
	// 		&format!("boolean"),
	// 		Object::BuiltinRust {
	// 			name: format!("boolean"),
	// 			len_args: 1,
	// 			builtinfn: builtins::boolean_builtin,
	// 		},
	// 	);
	// }

	fn time(env: &mut Env) {
		env.set(
			&format!("sleep"),
			Arc::new(Mutex::new(Box::new(FunctionRust::new(sleep)))),
		);
		env.set(
			&format!("ard"),
			Arc::new(Mutex::new(Box::new(FunctionRust::new(arduino)))),
		);
		// env.set(
		// 	&format!("ard"),
		// 	Object::BuiltinRust {
		// 		name: format!("ard"),
		// 		len_args: 0,
		// 		builtinfn: time::arduino,
		// 	},
		// );
	}
}

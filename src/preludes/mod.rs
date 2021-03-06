// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

mod input;
mod print_println;
mod type_object;

pub fn load(env: &mut Env) {
	env.set("print", Object::Builtin(format!("print"), -1, print_println::print));
	env.set("println", Object::Builtin(format!("println"), -1, print_println::println));
	env.set("input", Object::Builtin(format!("input"), -1, input::input));
	env.set("type", Object::Builtin(format!("type"), 1, type_object::type_object));
}
